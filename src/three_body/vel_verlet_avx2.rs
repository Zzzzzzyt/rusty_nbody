#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use std::mem::{transmute, MaybeUninit};

use crate::{util, vec3::Vec3};

use super::State3;

impl From<Vec3> for __m256d {
    fn from(vec: Vec3) -> Self {
        unsafe { _mm256_set_pd(vec.x, vec.y, vec.z, 0.0) }
    }
}

fn splat3(v: f64) -> __m256d {
    unsafe { _mm256_set_pd(v, v, v, 0.0) }
}

// fn print_m256d(m256d: __m256d) {
//     unsafe {
//         let buf: Aligned32 = MaybeUninit::uninit().assume_init();
//         let mut buf = transmute::<Aligned32, [f64; 4]>(buf);
//         _mm256_store_pd(&mut buf as *mut f64, m256d);
//         println!("{:?}", buf);
//     }
// }

#[repr(align(32))]
struct Aligned32([f64; 4]);

impl From<__m256d> for Vec3 {
    #[allow(invalid_value)]
    fn from(m256d: __m256d) -> Self {
        unsafe {
            let buf: Aligned32 = MaybeUninit::uninit().assume_init();
            let mut buf = transmute::<Aligned32, [f64; 4]>(buf);
            _mm256_store_pd(&mut buf as *mut f64, m256d);
            Vec3 {
                x: buf[3],
                y: buf[2],
                z: buf[1],
            }
        }
    }
}

#[inline(always)]
pub fn kernel(initial: State3, dt: u64, steps: u64) -> State3 {
    unsafe {
        let dtf = dt as f64;

        let mut v0 = __m256d::from(initial.v[0] * dtf);
        let mut v1 = __m256d::from(initial.v[1] * dtf);
        let mut v2 = __m256d::from(initial.v[2] * dtf);

        let mut p0 = __m256d::from(initial.p[0]);
        let mut p1 = __m256d::from(initial.p[1]);
        let mut p2 = __m256d::from(initial.p[2]);

        let MODIFIED_GRAVITY_CONSTANT = util::GRAVITY_CONSTANT * dtf * dtf;
        let mm0 = splat3(initial.m[0] * MODIFIED_GRAVITY_CONSTANT);
        let mm1 = splat3(initial.m[1] * MODIFIED_GRAVITY_CONSTANT);
        let mm2 = splat3(initial.m[2] * MODIFIED_GRAVITY_CONSTANT);

        // let mut dtm = _mm256_set1_pd(dt2);

        for _step in 0..steps {
            // advance(&mut v, a, dt2m);
            // v0 = _mm256_fmadd_pd(dtm, a0, v0);
            // v1 = _mm256_fmadd_pd(dtm, a1, v1);
            // v2 = _mm256_fmadd_pd(dtm, a2, v2);
            // // v0 = _mm256_add_pd(v0, a0);
            // v1 = _mm256_add_pd(v1, a1);
            // v2 = _mm256_add_pd(v2, a2);

            // advance(&mut p, v, dtm);
            // dtm = _mm256_set1_pd(dt);
            // p0 = _mm256_fmadd_pd(dtm, v0, p0);
            // p1 = _mm256_fmadd_pd(dtm, v1, p1);
            // p2 = _mm256_fmadd_pd(dtm, v2, p2);
            p0 = _mm256_add_pd(p0, v0);
            p1 = _mm256_add_pd(p1, v1);
            p2 = _mm256_add_pd(p2, v2);

            let r01 = _mm256_sub_pd(p1, p0);
            let r12 = _mm256_sub_pd(p2, p1);
            let r20 = _mm256_sub_pd(p0, p2);

            // r01 x
            let mag1 = calc_mag(r01);
            // r12 x
            let mag2 = calc_mag(r12);

            // r01 r12
            let mag_acc = _mm_shuffle_pd::<0b00>(mag1, mag2);
            let mag_acc = _mm_sqrt_pd(mag_acc);

            let mag1 = _mm_mul_sd(mag1, mag_acc);
            let mb1 = _mm256_broadcastsd_pd(mag1);
            let r01 = _mm256_div_pd(r01, mb1);

            // r12 r12
            let mag_acc_hi = _mm_unpackhi_pd(mag_acc, mag_acc);
            let mag2 = _mm_mul_sd(mag2, mag_acc_hi);
            let mb2 = _mm256_broadcastsd_pd(mag2);
            let r12 = _mm256_div_pd(r12, mb2);

            // r20 x
            let mag3 = calc_mag(r20);
            let mag3_sqrt = _mm_sqrt_sd(mag3, mag3);
            let mag3 = _mm_mul_sd(mag3, mag3_sqrt);
            let mb3 = _mm256_broadcastsd_pd(mag3);
            let r20 = _mm256_div_pd(r20, mb3);

            let a0 = _mm256_mul_pd(r01, mm1);
            let a0 = _mm256_fnmadd_pd(r20, mm2, a0);

            let a1 = _mm256_mul_pd(r12, mm2);
            let a1 = _mm256_fnmadd_pd(r01, mm0, a1);

            let a2 = _mm256_mul_pd(r20, mm0);
            let a2 = _mm256_fnmadd_pd(r12, mm1, a2);

            // advance(&mut v, a, dt2m);
            // dtm = _mm256_set1_pd(dt2);
            // v0 = _mm256_fmadd_pd(dtm, a0, v0);
            // v1 = _mm256_fmadd_pd(dtm, a1, v1);
            // v2 = _mm256_fmadd_pd(dtm, a2, v2);
            v0 = _mm256_add_pd(v0, a0);
            v1 = _mm256_add_pd(v1, a1);
            v2 = _mm256_add_pd(v2, a2);
        }

        State3 {
            p: [Vec3::from(p0), Vec3::from(p1), Vec3::from(p2)],
            v: [
                Vec3::from(v0) / dtf,
                Vec3::from(v1) / dtf,
                Vec3::from(v2) / dtf,
            ],
            m: initial.m,
            t: initial.t + (steps * dt),
        }
    }
}

#[inline(always)]
unsafe fn calc_mag(r: __m256d) -> __m128d {
    let r2 = _mm256_mul_pd(r, r);

    let low128 = _mm256_castpd256_pd128(r2);
    let high128 = _mm256_extractf128_pd::<1>(r2);
    let add1 = _mm_add_pd(low128, high128);
    let high64 = _mm_unpackhi_pd(add1, add1);
    let mag = _mm_add_pd(add1, high64);

    // let sqrtmag = _mm_sqrt_sd(mag, mag);
    // let mag = _mm_mul_sd(mag, sqrtmag);
    // let mag = _mm256_broadcastsd_pd(mag);
    // let ans = _mm256_div_pd(r, mag);

    return mag;
}
