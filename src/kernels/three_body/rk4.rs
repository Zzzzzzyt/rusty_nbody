use crate::Vec3;

use super::*;
use crate::util;

pub struct RK4Kernel;

impl ThreeBodyKernel for RK4Kernel {
    fn kernel(state: ThreeBodyState, steps: u64, dt: u64) -> ThreeBodyState {
        let dtf = dt as f64 * util::UNIT_TIME;

        let original_m = state.m.clone();
        let m = [
            Vec3::splat(state.m[0] * util::GRAVITY_CONSTANT),
            Vec3::splat(state.m[1] * util::GRAVITY_CONSTANT),
            Vec3::splat(state.m[2] * util::GRAVITY_CONSTANT),
        ];

        let dtm = Vec3::splat(dtf);
        let dtm2 = Vec3::splat(dtf / 2.0);
        let dtm3 = Vec3::splat(dtf / 3.0);
        let dtm6 = Vec3::splat(dtf / 6.0);

        let mut p = state.p;
        let mut v = state.v;

        for _ in 0..steps {
            let k1r = v;
            let k1v = calc_a(&p, &m);

            let p1 = advance(&p, &v, &dtm2);
            let k2r = advance(&v, &k1v, &dtm2);
            let k2v = calc_a(&p1, &m);

            let p2 = advance(&p, &k2r, &dtm2);
            let k3r = advance(&v, &k2v, &dtm2);
            let k3v = calc_a(&p2, &m);

            let p3 = advance(&p, &k3r, &dtm);
            let k4r = advance(&v, &k3v, &dtm);
            let k4v = calc_a(&p3, &m);

            v = advance(&v, &k1v, &dtm6);
            v = advance(&v, &k2v, &dtm3);
            v = advance(&v, &k3v, &dtm3);
            v = advance(&v, &k4v, &dtm6);

            p = advance(&p, &k1r, &dtm6);
            p = advance(&p, &k2r, &dtm3);
            p = advance(&p, &k3r, &dtm3);
            p = advance(&p, &k4r, &dtm6);
        }
        ThreeBodyState {
            p: p,
            v: v,
            m: original_m,
            t: state.t + steps * dt,
        }
    }
}
