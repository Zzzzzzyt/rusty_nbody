use super::State3;
use crate::{util, vec3::Vec3};

#[inline(always)]
pub fn kernel(initial: State3, dt: u64, steps: u64) -> State3 {
    let mut state = initial;
    let dtf = dt as f64;
    let dtf2 = dtf * 0.5;
    let m0 = state.m[0] * util::GRAVITY_CONSTANT;
    let m1 = state.m[1] * util::GRAVITY_CONSTANT;
    let m2 = state.m[2] * util::GRAVITY_CONSTANT;
    let mut a = [Vec3::default(); 3];
    for _ in 0..steps {
        advance(&mut state.v, &a, dtf2);

        advance(&mut state.p, &state.v, dtf);

        a = calc_a(&state.p, m0, m1, m2);

        advance(&mut state.v, &a, dtf2);

        // println!("{:?}", state.p[0]);
    }
    state.m = initial.m;
    state.t = initial.t + steps * dt;
    return state;
}

#[inline(always)]
pub fn calc_r(p1: Vec3, p2: Vec3) -> Vec3 {
    let r = p2 - p1;
    let r2 = r.len2();
    let mag = r2 * r2.sqrt();
    return r / mag;
}

#[inline(always)]
pub fn calc_a(p: &[Vec3; 3], m0: f64, m1: f64, m2: f64) -> [Vec3; 3] {
    let r01 = calc_r(p[0], p[1]);
    let r12 = calc_r(p[1], p[2]);
    let r20 = calc_r(p[2], p[0]);

    let a0 = r01 * m1 - r20 * m2;
    let a1 = r12 * m2 - r01 * m0;
    let a2 = r20 * m0 - r12 * m1;

    return [a0, a1, a2];
}

#[inline(always)]
pub fn advance(x: &mut [Vec3], v: &[Vec3], dt: f64) {
    for i in 0..3 {
        x[i] += v[i] * dt;
    }
}
