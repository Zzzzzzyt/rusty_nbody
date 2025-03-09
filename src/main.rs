pub mod kernels;
mod macros;
pub mod test;
pub mod util;
mod vec3;
pub mod viewer;

use std::time::Instant;

use kernels::three_body::*;
pub use vec3::*;

fn main() {
    // let p = vec![
    //     Vec3::new(0.0, 1e10, 0.0),
    //     Vec3::new(0.5e11, 0.0, 0.0),
    //     Vec3::new(1e11, -1e10, 0.0),
    // ];
    // let v = vec![
    //     Vec3::new(0.0, -10000.0, 0.0),
    //     Vec3::new(0.0, 10000.0, 28000.0),
    //     Vec3::new(0.0, 0.0, -28000.0),
    // ];
    // let m = vec![7e29, 7e29, 7e29];
    // let p = vec![
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(149.6e9, 0.0, 0.0),
    //     Vec3::new(149.6e9 + 0.384e9, 0.0, 0.0),
    // ];
    // let v = vec![
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(0.0, 0.0, 29.8e3),
    //     Vec3::new(0.0, 0.0, 29.8e3 + 1.0e3),
    // ];
    // let m = vec![1.989e30, 5.972e24, 7.3477e22];

    let p = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1e10, 0.0, 0.0),
        Vec3::new(0.0, -1e12, 0.0),
    ];
    let v = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.5e5),
        Vec3::new(10000.0, 0.0, 0.0),
    ];
    let m = vec![2e30, 0.0, 0.0];
    let mut state = kernels::PhysicsState { p, v, m, t: 0 };
    // state.normalize();

    // let mut ground_truth = state.clone();
    // simulate(yoshida4_relative_kernel, &mut ground_truth, 10000, 10000, 1);

    // viewer::start_viewer::<Yoshida4RelativeKernel>(state, 100, 200000);

    test::test_error::<Yoshida4RelativeKernel>(&state, "analysis/yoshida4_relative.json");
    test::test_error::<Yoshida4Kernel>(&state, "analysis/yoshida4.json");
    test::test_error::<VelVerletRelativeKernel>(&state, "analysis/vel_verlet_relative.json");
    test::test_error::<VelVerletKernel>(&state, "analysis/vel_verlet.json");
    test::test_error::<SymplecticEulerRelativeKernel>(
        &state,
        "analysis/symplectic_euler_relative.json",
    );
    test::test_error::<SymplecticEulerKernel>(&state, "analysis/symplectic_euler.json");
    test::test_error::<RK4Kernel>(&state, "analysis/rk4.json");
}
