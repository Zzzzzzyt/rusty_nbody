use std::time::Instant;

use crate::{
    kernels::{three_body::*, PhysicsState},
    util,
};
use kiss3d::{
    nalgebra::{Point3, Translation3, Vector3},
    scene::SceneNode,
    window::{CanvasSetup, NumSamples, State, Window},
};
use palette::{FromColor, Hsv, Srgb};

fn get_color(hue: f32) -> Point3<f32> {
    let hsv = Hsv::new(hue * 360.0, 0.8, 1.0);
    let rgb = Srgb::from_color(hsv);
    return Point3::new(rgb.red, rgb.green, rgb.blue);
}

struct ViewerState {
    state0: PhysicsState,
    state: PhysicsState,
    objs: Vec<SceneNode>,
    trails: Vec<Vec<Vector3<f32>>>,
    kernel: ThreeBodyKernelFn,
    step_count: u64,
    dt: u64,
}

impl State for ViewerState {
    fn step(&mut self, window: &mut Window) {
        let now1 = Instant::now();
        simulate(self.kernel, &mut self.state, 1, self.step_count, self.dt);

        println!("real_time: {}ns", now1.elapsed().as_nanos());
        println!(
            "speedup: {:.0}x",
            (self.step_count * self.dt) as f64 * util::UNIT_TIME / now1.elapsed().as_secs_f64()
        );

        self.state.print_summary();
        self.state.print_errors(&self.state0);
        println!();

        for i in 0..3 {
            let mut p = self.state.p[i];
            p /= 1e11;
            let tmp: [f64; 3] = p.into();
            let p2 = Vector3::<f32>::new(tmp[0] as f32, tmp[1] as f32, tmp[2] as f32);
            self.trails[i].push(p2);
            self.objs[i].set_local_translation(Translation3::from(p2));
            let color = get_color((i as f32) / 3.0);
            for j in 0..(self.trails[i].len() - 1) {
                window.draw_line(
                    &Point3::from(self.trails[i][j]),
                    &Point3::from(self.trails[i][j + 1]),
                    &Point3::new(color.x, color.y, color.z),
                );
            }
        }
    }

    fn cameras_and_effect(
        &mut self,
    ) -> (
        Option<&mut dyn kiss3d::camera::Camera>,
        Option<&mut dyn kiss3d::planar_camera::PlanarCamera>,
        Option<&mut dyn kiss3d::post_processing::PostProcessingEffect>,
    ) {
        (None, None, None)
    }

    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn kiss3d::camera::Camera>,
        Option<&mut dyn kiss3d::planar_camera::PlanarCamera>,
        Option<&mut dyn kiss3d::renderer::Renderer>,
        Option<&mut dyn kiss3d::post_processing::PostProcessingEffect>,
    ) {
        #[allow(deprecated)]
        let res = self.cameras_and_effect(); // For backward-compatibility.
        (res.0, res.1, None, res.2)
    }
}

pub fn start_viewer(kernel: ThreeBodyKernelFn, state: PhysicsState, dt: u64, step_count: u64) {
    let mut window = Window::new_with_setup(
        "Rusty NBody",
        1440,
        900,
        CanvasSetup {
            vsync: true,
            samples: NumSamples::Four,
        },
    );

    let mut viewer_state = ViewerState {
        state0: state.clone(),
        state: state,
        objs: vec![],
        trails: vec![],
        kernel: kernel,
        step_count: step_count,
        dt: dt,
    };

    for _ in 0..3 {
        viewer_state.objs.push(window.add_sphere(0.02));
        viewer_state.trails.push(vec![]);
    }
    window.set_framerate_limit(None);
    window.render_loop(viewer_state);
}
