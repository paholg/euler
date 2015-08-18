#![feature(core, step_by, collections)]

extern crate primal;
extern crate num;
extern crate time;
extern crate itertools;
extern crate core;

use time::PreciseTime;

mod p001;
use p001::*;
mod p011;
use p011::*;
mod p021;
use p021::*;
mod p031;
use p031::*;

pub mod funs {
    use num::traits::PrimInt;
    pub fn to_digits(n: u64) -> Vec<u64> {
        (1..ndigits(n)+1).rev().map(|i| n % 10.pow(i) / 10.pow(i-1)).collect::<Vec<u64>>()
    }
    pub fn ndigits(n: u64) -> u32 {
        (1..).take_while(|&x| n / 10.pow(x) > 0).last().unwrap_or(0) + 1
    }
}

fn main() {
    let start = 30;

    let probs: &[fn() -> u64] = &[
        p1,  p2,  p3,  p4,  p5,  p6,  p7,  p8,  p9,  p10,
        p11, p12, p13, p14, p15, p16, p17, p18, p19, p20,
        p21, p22, p23, p24, p25, p26, p27, p28, p29, p30,
        p31, p32, p33, p34, p35, p36, p37, p38, p39, p40,
            ];

    for (n, p) in probs.into_iter().skip(start-1).enumerate() {
        let before = PreciseTime::now();
        let result = p();
        let after = PreciseTime::now();
        let dur = before.to(after).num_milliseconds();
        println!("Problem {:3} (took {:4} ms): {}", n+start, dur, result);
    }
}
