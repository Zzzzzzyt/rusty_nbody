#![allow(non_snake_case)]

pub mod three_body;
pub mod util;
pub mod vec3;

use std::time::Instant;

use three_body::{rk4, vel_verlet, vel_verlet_avx2, yoshida4, State3};

use rand::prelude::*;
use vec3::Vec3;

fn rand_vector(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(min..max);
    let y = rng.gen_range(min..max);
    let z = rng.gen_range(min..max);
    Vec3::init(x, y, z)
}

fn main() {
    let p = [
        Vec3::init(0.0, 1e10, 0.0),
        Vec3::init(0.5e11, 0.0, 0.0),
        Vec3::init(1e11, -1e10, 0.0),
    ];
    let v = [
        Vec3::init(0.0, -10000.0, 0.0),
        Vec3::init(0.0, 10000.0, 28000.0),
        Vec3::init(0.0, 0.0, -28000.0),
    ];
    let m = [7e29, 7e29, 7e29];
    let state1 = State3 { p, v, m, t: 0 };

    let N = 3155673600;
    let dt = 1;

    state1.print_summary();
    println!();

    let now1 = Instant::now();
    let state2 = vel_verlet_avx2::kernel(state1, dt, N);
    println!("simple_avx2: {}ns", now1.elapsed().as_nanos());
    // println!("{:#?}", state2);
    state2.print_summary();
    println!(
        "diff: {:.10e}\n",
        state2.calc_total_energy() - state1.calc_total_energy()
    );

    let now2 = Instant::now();
    let state3 = vel_verlet::kernel(state1, dt, N);
    println!("simple: {}ns", now2.elapsed().as_nanos());
    // println!("{:#?}", state3);
    state3.print_summary();
    println!(
        "diff: {:.10e}\n",
        state3.calc_total_energy() - state1.calc_total_energy()
    );

    let now3 = Instant::now();
    let state4 = yoshida4::kernel(state1, dt, N);
    println!("yoshida4: {}ns", now3.elapsed().as_nanos());
    // println!("{:#?}", state3);
    state4.print_summary();
    println!(
        "diff: {:.10e}\n",
        state4.calc_total_energy() - state1.calc_total_energy()
    );

    let now4 = Instant::now();
    let state5 = rk4::kernel(state1, dt, N);
    println!("rk4: {}ns", now4.elapsed().as_nanos());
    // println!("{:#?}", state3);
    state5.print_summary();
    println!(
        "diff: {:.10e}\n",
        state5.calc_total_energy() - state1.calc_total_energy()
    );
}
