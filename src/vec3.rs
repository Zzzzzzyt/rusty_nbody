#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::fmt::Debug;
use core::fmt::Display;
use core::ops::*;

#[cfg(target_feature = "avx2")]
#[repr(C, align(32))]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub avx: __m256d,
}

#[cfg(not(target_feature = "avx2"))]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[cfg(target_feature = "avx2")]
impl Vec3 {
    #[must_use]
    #[inline]
    const fn from_array(arr: [f64; 4]) -> Self {
        unsafe { core::mem::transmute(arr) }
    }

    #[must_use]
    #[inline]
    fn to_array(&self) -> [f64; 4] {
        unsafe { core::mem::transmute(*self) }
    }
}

impl Vec3 {
    #[cfg(target_feature = "avx2")]
    #[must_use]
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            avx: unsafe { core::mem::transmute([x, y, z, 0.0]) },
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[must_use]
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn splat(value: f64) -> Self {
        Self::new(value, value, value)
    }
}

impl From<Vec3> for [f64; 3] {
    #[cfg(target_feature = "avx2")]
    #[must_use]
    #[inline]
    fn from(value: Vec3) -> Self {
        let arr = value.to_array();
        [arr[0], arr[1], arr[2]]
    }

    #[cfg(not(target_feature = "avx2"))]
    #[must_use]
    #[inline]
    fn from(value: Vec3) -> Self {
        [value.x, value.y, value.z]
    }
}

#[cfg(target_feature = "avx2")]
impl Vec3 {
    pub const ZERO: Vec3 = Vec3::from_array([0.0; 4]);
    pub const ONE: Vec3 = Vec3::from_array([1.0; 4]);
}

#[cfg(not(target_feature = "avx2"))]
impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

#[cfg(target_feature = "avx2")]
impl Default for Vec3 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Vec3 {
            avx: unsafe { _mm256_setzero_pd() },
        }
    }
}

#[cfg(target_feature = "avx2")]
impl PartialEq for Vec3 {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let cmp1 = _mm256_cmp_pd(self.avx, other.avx, _CMP_EQ_OQ);
            let mask = _mm256_movemask_pd(cmp1);
            mask == 0b1111
        }
    }
}

impl Debug for Vec3 {
    #[cfg(target_feature = "avx2")]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let arr: [f64; 4] = self.to_array();
        write!(f, "Vec3({:.10e}, {:.10e}, {:.10e})", arr[0], arr[1], arr[2])
    }

    #[cfg(not(target_feature = "avx2"))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec3({:.10e}, {:.10e}, {:.10e})", self.x, self.y, self.z)
    }
}

impl Display for Vec3 {
    #[cfg(target_feature = "avx2")]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let arr: [f64; 4] = self.to_array();
        write!(f, "({:.10e}, {:.10e}, {:.10e})", arr[0], arr[1], arr[2])
    }

    #[cfg(not(target_feature = "avx2"))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:.10e}, {:.10e}, {:.10e})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        unsafe {
            Vec3 {
                avx: _mm256_add_pd(self.avx, rhs.avx),
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        unsafe {
            Vec3 {
                avx: _mm256_sub_pd(self.avx, rhs.avx),
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    fn mul(self, rhs: f64) -> Self {
        unsafe {
            let scalar = _mm256_set1_pd(rhs);
            Vec3 {
                avx: _mm256_mul_pd(self.avx, scalar),
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    fn mul(self, rhs: Self) -> Self {
        unsafe {
            Vec3 {
                avx: _mm256_mul_pd(self.avx, rhs.avx),
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    fn div(self, rhs: f64) -> Self {
        unsafe {
            let scalar = _mm256_set1_pd(rhs);
            Vec3 {
                avx: _mm256_div_pd(self.avx, scalar),
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    fn div(self, rhs: Self) -> Self {
        unsafe {
            Vec3 {
                avx: _mm256_div_pd(self.avx, rhs.avx),
            }
        }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    fn div(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn neg(self) -> Self {
        Self::ZERO - self
    }
}

#[cfg(target_feature = "avx2")]
#[inline]
#[must_use]
unsafe fn reduce_add0(m: __m256d) -> __m128d {
    let low128 = _mm256_castpd256_pd128(m);
    let high128 = _mm256_extractf128_pd::<1>(m);
    let add1 = _mm_add_pd(low128, high128);
    let high64 = _mm_unpackhi_pd(add1, add1);
    let mag = _mm_add_pd(add1, high64);
    mag
}

impl Vec3 {
    #[cfg(target_feature = "avx2")]
    #[inline]
    #[must_use]
    pub fn reduce_add(&self) -> f64 {
        unsafe { _mm_cvtsd_f64(reduce_add0(self.avx)) }
    }

    #[cfg(not(target_feature = "avx2"))]
    #[inline]
    #[must_use]
    pub fn reduce_add(&self) -> f64 {
        self.x + self.y + self.z
    }

    #[inline]
    #[must_use]
    pub fn norm_squared(&self) -> f64 {
        let squared = self * self;
        squared.reduce_add()
    }

    #[inline]
    #[must_use]
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }
}

impl Vec3 {
    #[cfg(all(target_feature = "avx2", target_feature = "fma"))]
    #[inline]
    #[must_use]
    pub fn mul_add(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                avx: _mm256_fmadd_pd(a.avx, b.avx, c.avx),
            }
        }
    }

    #[cfg(not(all(target_feature = "avx2", target_feature = "fma")))]
    #[inline]
    #[must_use]
    pub fn mul_add(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        a * b + c
    }

    #[cfg(all(target_feature = "avx2", target_feature = "fma"))]
    #[inline]
    #[must_use]
    pub fn mul_sub(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                avx: _mm256_fmsub_pd(a.avx, b.avx, c.avx),
            }
        }
    }

    #[cfg(not(all(target_feature = "avx2", target_feature = "fma")))]
    #[inline]
    #[must_use]
    pub fn mul_sub(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        a * b - c
    }

    #[cfg(all(target_feature = "avx2", target_feature = "fma"))]
    #[inline]
    #[must_use]
    pub fn mul_neg_add(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                avx: _mm256_fnmadd_pd(a.avx, b.avx, c.avx),
            }
        }
    }

    #[cfg(not(all(target_feature = "avx2", target_feature = "fma")))]
    #[inline]
    #[must_use]
    pub fn mul_neg_add(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        -a * b + c
    }

    #[cfg(all(target_feature = "avx2", target_feature = "fma"))]
    #[inline]
    #[must_use]
    pub fn mul_neg_sub(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        unsafe {
            Vec3 {
                avx: _mm256_fnmsub_pd(a.avx, b.avx, c.avx),
            }
        }
    }

    #[cfg(not(all(target_feature = "avx2", target_feature = "fma")))]
    #[inline]
    #[must_use]
    pub fn mul_neg_sub(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        -a * b - c
    }

    #[inline]
    #[must_use]
    pub fn calc_r(p1: &Vec3, p2: &Vec3) -> Vec3 {
        let r = p2 - p1;
        let r2 = r.norm_squared();
        let mag = r2 * r2.sqrt();
        return r / mag;
    }
}
