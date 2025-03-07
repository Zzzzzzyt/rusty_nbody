use crate::util;
use crate::Vec3;

use super::*;

#[inline(always)]
pub fn vel_verlet_kernel(state: ThreeBodyState, steps: u64, dt: u64) -> ThreeBodyState {
    let original_m = state.m.clone();

    let dtf = dt as f64 * util::UNIT_TIME;
    let modified_g = util::GRAVITY_CONSTANT * dtf * dtf;

    let m = [
        Vec3::splat(state.m[0] * modified_g),
        Vec3::splat(state.m[1] * modified_g),
        Vec3::splat(state.m[2] * modified_g),
    ];
    let mut p = state.p;
    let mut v = state.v;
    v[0] *= dtf;
    v[1] *= dtf;
    v[2] *= dtf;
    for _ in 0..steps {
        p = add(&p, &v);
        let a = calc_a(&p, &m);
        v = add(&v, &a);
    }
    v[0] /= dtf;
    v[1] /= dtf;
    v[2] /= dtf;
    ThreeBodyState {
        p: p,
        v: v,
        m: original_m,
        t: state.t + steps * dt,
    }
}

#[inline(always)]
pub fn vel_verlet_relative_kernel(state: ThreeBodyState, steps: u64, dt: u64) -> ThreeBodyState {
    let original_m = state.m.clone();

    let dtf = dt as f64 * util::UNIT_TIME;
    let modified_g = util::GRAVITY_CONSTANT * dtf * dtf;

    let m = [
        Vec3::splat(state.m[0] * modified_g),
        Vec3::splat(state.m[1] * modified_g),
        Vec3::splat(state.m[2] * modified_g),
    ];
    let mut p0 = state.p;
    let mut v0 = [state.v[0] * dtf, state.v[1] * dtf, state.v[2] * dtf];

    let mut p = [Vec3::ZERO; 3];
    let mut v = [Vec3::ZERO; 3];

    for _ in 0..steps {
        p = add(&p, &v);
        p = add(&p, &v0);
        let a = calc_a(&add(&p0, &p), &m);
        v = add(&v, &a);
    }

    v0 = add(&v0, &v);
    v0[0] /= dtf;
    v0[1] /= dtf;
    v0[2] /= dtf;

    p0 = add(&p0, &p);

    ThreeBodyState {
        p: p0,
        v: v0,
        m: original_m,
        t: state.t + steps * dt,
    }
}
