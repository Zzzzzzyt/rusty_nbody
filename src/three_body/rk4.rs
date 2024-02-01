use super::vel_verlet::calc_a;
use super::State3;
use crate::util;
use crate::vec3::Vec3;

#[inline(always)]
pub fn kernel(initial: State3, dt: u64, steps: u64) -> State3 {
    let dtf = dt as f64;

    let mut state = initial;
    let m0 = state.m[0] * util::GRAVITY_CONSTANT;
    let m1 = state.m[1] * util::GRAVITY_CONSTANT;
    let m2 = state.m[2] * util::GRAVITY_CONSTANT;

    let dtf2 = dtf / 2.0;
    let dtf3 = dtf / 3.0;
    let dtf6 = dtf / 6.0;

    for _ in 0..steps {
        let mut v0 = state.v;
        let mut r0 = state.p;

        let k1r = state.v;
        let k1v = calc_a(&r0, m0, m1, m2);

        let r1 = fmadd(&r0, &k1r, dtf2);
        let k2r = fmadd(&v0, &k1v, dtf2);
        let k2v = calc_a(&r1, m0, m1, m2);

        let r2 = fmadd(&r0, &k2r, dtf2);
        let k3r = fmadd(&v0, &k2v, dtf2);
        let k3v = calc_a(&r2, m0, m1, m2);

        let r3 = fmadd(&r0, &k3r, dtf);
        let k4r = fmadd(&v0, &k3v, dtf);
        let k4v = calc_a(&r3, m0, m1, m2);

        v0 = fmadd(&v0, &k1v, dtf6);
        v0 = fmadd(&v0, &k2v, dtf3);
        v0 = fmadd(&v0, &k3v, dtf3);
        v0 = fmadd(&v0, &k4v, dtf6);

        r0 = fmadd(&r0, &k1r, dtf6);
        r0 = fmadd(&r0, &k2r, dtf3);
        r0 = fmadd(&r0, &k3r, dtf3);
        r0 = fmadd(&r0, &k4r, dtf6);

        state.v = v0;
        state.p = r0;
    }
    state.m = initial.m;
    state.t = initial.t + steps * dt;
    return state;
}

fn fmadd(a: &[Vec3; 3], b: &[Vec3; 3], c: f64) -> [Vec3; 3] {
    return [a[0] + b[0] * c, a[1] + b[1] * c, a[2] + b[2] * c];
}
