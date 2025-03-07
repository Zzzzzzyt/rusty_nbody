use core::mem::transmute;

use super::*;
use crate::{util, Vec3};

// w0 = -2^(1/3)/(2-2^(1/3))
//    ~= -1.702414383919315268095375617942921653843998752434289656657411995...
//    ~= 0x3ff59e8b6eb96339
// w1 = 1/(2-2^(1/3))
//    ~= 1.3512071919596576340476878089714608269219993762171448283287059976...
//    ~= 0xbffb3d16dd72c672

// d1 = w1
const D1: f64 = unsafe { transmute::<u64, f64>(0x3ff59e8b6eb96339) };
// d2 = w0
const D2: f64 = unsafe { transmute::<u64, f64>(0xbffb3d16dd72c672) };
// d3 = w1
const D3: f64 = D1;

// c1 = w1/2
//    ~= 0.6756035959798288170238439044857304134609996881085724141643529988...
//    ~= 0x3fe59e8b6eb96339
const C1: f64 = unsafe { transmute::<u64, f64>(0x3fe59e8b6eb96339) };

// c2 = (w0+w1)/2
//    ~= -0.175603595979828817023843904485730413460999688108572414164352998...
//    ~=  0xbfc67a2dbae58ce4
const C2: f64 = unsafe { transmute::<u64, f64>(0xbfc67a2dbae58ce4) };

// c3 = (w0+w1)/2
const C3: f64 = C2;

// c4 = w1/2
const C4: f64 = C1;

#[inline(always)]
pub fn yoshida4_kernel(state: ThreeBodyState, steps: u64, dt: u64) -> ThreeBodyState {
    let dtf = dt as f64 * util::UNIT_TIME;

    let original_m = state.m.clone();
    let m = [
        Vec3::splat(state.m[0] * util::GRAVITY_CONSTANT),
        Vec3::splat(state.m[1] * util::GRAVITY_CONSTANT),
        Vec3::splat(state.m[2] * util::GRAVITY_CONSTANT),
    ];

    let c1 = Vec3::splat(C1 * dtf);
    let c2 = Vec3::splat(C2 * dtf);
    let c3 = Vec3::splat(C3 * dtf);
    let c4 = Vec3::splat(C4 * dtf);

    let d1 = Vec3::splat(D1 * dtf);
    let d2 = Vec3::splat(D2 * dtf);
    let d3 = Vec3::splat(D3 * dtf);

    // println!("{} {} {} {}", c1, c2, c3, c4);
    // println!("{} {} {}", d1, d2, d3);

    let mut p = state.p;
    let mut v = state.v;

    for _ in 0..steps {
        p = advance(&p, &v, &c1);
        let a = calc_a(&p, &m);
        v = advance(&v, &a, &d1);

        p = advance(&p, &v, &c2);
        let a = calc_a(&p, &m);
        v = advance(&v, &a, &d2);

        p = advance(&p, &v, &c3);
        let a = calc_a(&p, &m);
        v = advance(&v, &a, &d3);

        p = advance(&p, &v, &c4);
    }

    ThreeBodyState {
        p: p,
        v: v,
        m: original_m,
        t: state.t + steps * dt,
    }
}

#[inline(always)]
pub fn yoshida4_relative_kernel(state: ThreeBodyState, steps: u64, dt: u64) -> ThreeBodyState {
    let dtf = dt as f64 * util::UNIT_TIME;

    let original_m = state.m.clone();
    let m = [
        Vec3::splat(state.m[0] * util::GRAVITY_CONSTANT),
        Vec3::splat(state.m[1] * util::GRAVITY_CONSTANT),
        Vec3::splat(state.m[2] * util::GRAVITY_CONSTANT),
    ];

    let c1 = Vec3::splat(C1 * dtf);
    let c2 = Vec3::splat(C2 * dtf);
    let c3 = Vec3::splat(C3 * dtf);
    let c4 = Vec3::splat(C4 * dtf);

    let d1 = Vec3::splat(D1 * dtf);
    let d2 = Vec3::splat(D2 * dtf);
    let d3 = Vec3::splat(D3 * dtf);

    // println!("{} {} {} {}", c1, c2, c3, c4);
    // println!("{} {} {}", d1, d2, d3);

    let p0 = state.p;
    let v0 = state.v;
    let mut p = [Vec3::ZERO; 3];
    let mut v = [Vec3::ZERO; 3];

    for _ in 0..steps {
        p = advance(&p, &add(&v0, &v), &c1);
        let a = calc_a(&add(&p0, &p), &m);
        v = advance(&v, &a, &d1);

        p = advance(&p, &add(&v0, &v), &c2);
        let a = calc_a(&add(&p0, &p), &m);
        v = advance(&v, &a, &d2);

        p = advance(&p, &add(&v0, &v), &c3);
        let a = calc_a(&add(&p0, &p), &m);
        v = advance(&v, &a, &d3);

        p = advance(&p, &add(&v0, &v), &c4);
    }

    ThreeBodyState {
        p: add(&p0, &p),
        v: add(&v0, &v),
        m: original_m,
        t: state.t + steps * dt,
    }
}
