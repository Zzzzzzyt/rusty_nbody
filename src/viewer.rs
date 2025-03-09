use std::time::{Duration, Instant};

use crate::{
    kernels::{three_body::*, PhysicsState},
    util,
};
use kiss3d::{
    camera::ArcBall,
    event::{Action, Key, WindowEvent},
    nalgebra::{Point2, Point3, Translation3, Vector3},
    scene::SceneNode,
    text::Font,
    window::{CanvasSetup, NumSamples, Window},
};
use palette::{Mix, Srgb};

fn get_color(hue: f32) -> Point3<f32> {
    let c1 = Srgb::new(0.8, 0.0, 0.0).into_linear();
    let c2 = Srgb::new(1.0, 0.7, 0.0).into_linear();
    let c3 = Srgb::new(0.0, 1.0, 0.0).into_linear();
    let c4 = Srgb::new(0.0, 0.0, 1.0).into_linear();
    let c5 = Srgb::new(1.0, 0.0, 1.0).into_linear();
    let color = if hue < 0.25 {
        c1.mix(c2, (hue - 0.0) / 0.25)
    } else if hue < 0.5 {
        c2.mix(c3, (hue - 0.25) / 0.25)
    } else if hue < 0.75 {
        c3.mix(c4, (hue - 0.5) / 0.25)
    } else {
        c4.mix(c5, (hue - 0.75) / 0.25)
    };
    let color = Srgb::from_linear(color);
    return Point3::new(color.red, color.green, color.blue);
}

fn render_grid(window: &mut Window, size: f32, y: f32) {
    let color = Point3::new(0.3, 0.3, 0.3);
    for i in -10..=10 {
        let i = i as f32 / 10.0 * size;
        window.draw_line(&Point3::new(i, y, -size), &Point3::new(i, y, size), &color);
        window.draw_line(&Point3::new(-size, y, i), &Point3::new(size, y, i), &color);
    }
}

pub fn start_viewer<T: ThreeBodyKernel>(initial_state: PhysicsState, dt: u64, step_count: u64) {
    let mut window = Window::new_with_setup(
        "Rusty NBody",
        1440,
        900,
        CanvasSetup {
            vsync: true,
            samples: NumSamples::Four,
        },
    );

    let mut camera = ArcBall::new(Point3::new(2.0, 2.0, 2.0), Point3::origin());

    let mut state = initial_state.clone();
    let mut objs: Vec<SceneNode> = vec![];
    let mut trails: Vec<Vec<Vector3<f32>>> = vec![];

    for i in 0..3 {
        let mut obj = window.add_sphere(0.03);
        let color = get_color(i as f32 / 3.0);
        obj.set_color(color.x, color.y, color.z);
        objs.push(obj);
        trails.push(vec![]);
    }

    window.set_framerate_limit(None);
    window.set_line_width(1.0);

    let mut last_time = Instant::now();
    let mut pause = false;
    let mut sim_time = Duration::default();
    let mut dt_ratio = 1.0;
    loop {
        let delta_time = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();

        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Press, _) => match key {
                    Key::Add | Key::Equals => {
                        // camera.set_dist(camera.dist() / 1.5);
                        dt_ratio *= 1.5;
                    }
                    Key::Subtract | Key::Minus => {
                        // camera.set_dist(camera.dist() * 1.5);
                        dt_ratio /= 1.5;
                    }
                    Key::Space => {
                        pause = !pause;
                    }
                    Key::Key0 | Key::Numpad0 => {
                        camera.look_at(Point3::new(2.0, 2.0, 2.0), Point3::origin());
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        camera.set_yaw(camera.yaw() + 0.1 * delta_time);

        if !pause {
            let timer = Instant::now();
            T::simulate(&mut state, 1, step_count, (dt as f64 * dt_ratio) as u64);
            sim_time = timer.elapsed();
        }

        println!("real_time: {}us", sim_time.as_micros());
        println!(
            "speedup: {:.0}x",
            (step_count * dt) as f64 * dt_ratio * util::UNIT_TIME / sim_time.as_secs_f64()
        );

        state.print_summary();
        state.print_errors(&initial_state);
        println!();

        render_grid(&mut window, 1.5, -0.5);
        for i in 0..3 {
            let mut p = state.p[i];
            p /= 1e11;
            let tmp: [f64; 3] = p.into();
            let p = Vector3::<f32>::new(tmp[0] as f32, tmp[1] as f32, tmp[2] as f32);
            trails[i].push(p);
            objs[i].set_local_translation(Translation3::from(p));

            let color = get_color((i as f32) / 3.0);
            window.draw_line(
                &Point3::from(p),
                &Point3::new(p.x, -0.5, p.z),
                &Point3::new(0.3, 0.3, 0.3),
            );
            for j in 0..(trails[i].len() - 1) {
                window.draw_line(
                    &Point3::from(trails[i][j]),
                    &Point3::from(trails[i][j + 1]),
                    &color,
                );
            }
        }
        let font = Font::default();
        window.draw_text(
            format!(
                "fps: {:.0}\nsim time: {}us\nspeedup: {:.0}x\ndt: {}\nreal time:{:.3}yr",
                1.0 / delta_time,
                sim_time.as_micros(),
                (step_count * dt) as f64 * dt_ratio * util::UNIT_TIME / sim_time.as_secs_f64(),
                (dt as f64 * dt_ratio) as u64,
                state.t as f64 * util::UNIT_TIME / util::YEAR,
            )
            .as_str(),
            &Point2::new(20.0, 10.0),
            60.0,
            &font,
            &Point3::new(1.0, 1.0, 1.0),
        );
        if !window.render_with_camera(&mut camera) {
            break;
        }
    }
}
