use std::mem::transmute;

use super::vel_verlet::{advance, calc_a};
use super::State3;
use crate::util;

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
pub fn kernel(initial: State3, dt: u64, steps: u64) -> State3 {
    let dtf = dt as f64;

    let mut state = initial;
    let m0 = state.m[0] * util::GRAVITY_CONSTANT;
    let m1 = state.m[1] * util::GRAVITY_CONSTANT;
    let m2 = state.m[2] * util::GRAVITY_CONSTANT;

    let c1 = C1 * dtf;
    let c2 = C2 * dtf;
    let c3 = C3 * dtf;
    let c4 = C4 * dtf;

    let d1 = D1 * dtf;
    let d2 = D2 * dtf;
    let d3 = D3 * dtf;

    // println!("{} {} {} {}", c1, c2, c3, c4);
    // println!("{} {} {}", d1, d2, d3);

    for _ in 0..steps {
        advance(&mut state.p, &state.v, c1);
        let a = calc_a(&state.p, m0, m1, m2);
        advance(&mut state.v, &a, d1);

        advance(&mut state.p, &state.v, c2);
        let a = calc_a(&state.p, m0, m1, m2);
        advance(&mut state.v, &a, d2);

        advance(&mut state.p, &state.v, c3);
        let a = calc_a(&state.p, m0, m1, m2);
        advance(&mut state.v, &a, d3);

        advance(&mut state.p, &state.v, c4);
    }
    state.m = initial.m;
    state.t = initial.t + dt * steps;
    return state;
}
