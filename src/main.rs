pub mod kernels;
pub mod macros;
pub mod util;
mod vec3;
pub mod viewer;

use std::time::Instant;

use kernels::three_body::*;
pub use vec3::*;

macro_rules! test_kernel {
    ($kernel:ident, $state:expr, $ground_truth:expr, $time:expr, $batch_time:expr, $dt:expr) => {{
        let time = $time;
        let batch_time = $batch_time;
        let dt = $dt;

        println!("---------------------------------------------------");
        println!("{}", stringify!($kernel));
        println!("time = {}, batch_time = {}, dt = {}", time, batch_time, dt);

        let state0 = $state.clone();
        let mut state1 = $state.clone();

        let now1 = Instant::now();
        simulate($kernel, &mut state1, time / batch_time, batch_time / dt, dt);
        println!("used: {}us", now1.elapsed().as_micros());
        println!();

        // state1.print_summary();
        // println!();
        state1.print_errors(&state0);
        println!();
        state1.print_deviation($ground_truth);
        println!("---------------------------------------------------");
        println!();
        println!();
    }};
}

fn main() {
    let p = vec![
        Vec3::new(0.0, 1e10, 0.0),
        Vec3::new(0.5e11, 0.0, 0.0),
        Vec3::new(1e11, -1e10, 0.0),
    ];
    let v = vec![
        Vec3::new(0.0, -10000.0, 0.0),
        Vec3::new(0.0, 10000.0, 28000.0),
        Vec3::new(0.0, 0.0, -28000.0),
    ];
    let m = vec![7e29, 7e29, 7e29];
    let mut state = kernels::PhysicsState { p, v, m, t: 0 };
    state.normalize();

    let mut ground_truth = state.clone();
    simulate(yoshida4_relative_kernel, &mut ground_truth, 10000, 10000, 1);

    test_kernel!(
        yoshida4_relative_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        1
    );
    test_kernel!(
        yoshida4_relative_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        10
    );
    test_kernel!(
        yoshida4_relative_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        100
    );
    test_kernel!(
        yoshida4_relative_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        1000
    );
    test_kernel!(
        yoshida4_relative_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        10000
    );

    test_kernel!(yoshida4_kernel, &state, &ground_truth, 100000000, 100000, 1);
    test_kernel!(
        yoshida4_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        10
    );
    test_kernel!(
        yoshida4_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        100
    );
    test_kernel!(
        yoshida4_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        1000
    );
    test_kernel!(
        yoshida4_kernel,
        &state,
        &ground_truth,
        100000000,
        100000,
        10000
    );

    // viewer::start_viewer(yoshida4_relative_kernel, state, 100, 100000);
}
