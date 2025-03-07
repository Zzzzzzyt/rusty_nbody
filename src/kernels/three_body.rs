use crate::*;

use super::PhysicsState;

mod rk4;
mod vel_verlet;
mod yoshida4;

pub use rk4::*;
pub use vel_verlet::*;
pub use yoshida4::*;

pub type ThreeBodyKernelFn = fn(ThreeBodyState, u64, u64) -> ThreeBodyState;

pub struct ThreeBodyState {
    pub p: [Vec3; 3],
    pub v: [Vec3; 3],
    pub m: [f64; 3],
    pub t: u64,
}

impl From<&PhysicsState> for ThreeBodyState {
    fn from(state: &PhysicsState) -> Self {
        if state.p.len() != 3 || state.v.len() != 3 || state.m.len() != 3 {
            panic!("PhysicsState must have exactly 3 bodies");
        }
        return ThreeBodyState {
            p: [state.p[0], state.p[1], state.p[2]],
            v: [state.v[0], state.v[1], state.v[2]],
            m: [state.m[0], state.m[1], state.m[2]],
            t: state.t,
        };
    }
}

impl From<&ThreeBodyState> for PhysicsState {
    fn from(state: &ThreeBodyState) -> Self {
        return PhysicsState {
            p: vec![state.p[0], state.p[1], state.p[2]],
            v: vec![state.v[0], state.v[1], state.v[2]],
            m: vec![state.m[0], state.m[1], state.m[2]],
            t: state.t,
        };
    }
}

pub fn simulate(
    kernel: ThreeBodyKernelFn,
    state: &mut PhysicsState,
    batch_count: u64,
    step_count: u64,
    dt: u64,
) {
    let mut state1 = ThreeBodyState::from(&*state);
    for _ in 0..batch_count {
        state1 = kernel(state1, step_count, dt);
    }
    *state = PhysicsState::from(&state1);
}

#[inline(always)]
#[must_use]
pub fn calc_a(p: &[Vec3; 3], m: &[Vec3; 3]) -> [Vec3; 3] {
    let r01 = Vec3::calc_r(&p[0], &p[1]);
    let r12 = Vec3::calc_r(&p[1], &p[2]);
    let r20 = Vec3::calc_r(&p[2], &p[0]);

    let a0 = r01 * m[1] - r20 * m[2];
    let a1 = r12 * m[2] - r01 * m[0];
    let a2 = r20 * m[0] - r12 * m[1];

    return [a0, a1, a2];
}

#[inline(always)]
#[must_use]
#[allow(dead_code)]
fn add(a: &[Vec3; 3], b: &[Vec3; 3]) -> [Vec3; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline(always)]
#[must_use]
#[allow(dead_code)]
fn sub(a: &[Vec3; 3], b: &[Vec3; 3]) -> [Vec3; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline(always)]
#[must_use]
#[allow(dead_code)]
fn mul(a: &[Vec3; 3], b: &[Vec3; 3]) -> [Vec3; 3] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}

#[inline(always)]
#[must_use]
#[allow(dead_code)]
fn advance(a: &[Vec3; 3], b: &[Vec3; 3], c: &Vec3) -> [Vec3; 3] {
    [
        Vec3::mul_add(b[0], *c, a[0]),
        Vec3::mul_add(b[1], *c, a[1]),
        Vec3::mul_add(b[2], *c, a[2]),
    ]
}
