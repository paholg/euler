use itertools::equal;
use primal::is_prime;
use num::{abs, pow};
use num::traits::{Float, PrimInt};
use num::bigint::BigUint;

pub fn p21() -> u64 {
    0
}
pub fn p22() -> u64 {
    0
}
pub fn p23() -> u64 {
    0
}
pub fn p24() -> u64 {
    0
}
pub fn p25() -> u64 {
    0
}
pub fn p26() -> u64 {
    let mut len = 1;
    let mut d = 1;
    for n in (1..1000) {
        let num = (1.0 / (n as f64)).to_string();
        let (lower, _) = num.as_bytes().iter().skip(2).map(|x| x-48).size_hint();
        for i in (1..lower) {
            let seq1 = num.as_bytes().iter().skip(2).map(|x| x-48).take(i);
            let seq2 = num.as_bytes().iter().skip(2).map(|x| x-48).skip(i).take(i);
            if equal(seq1, seq2) {
                if i > len {
                    len = i;
                    d = n
                }
                // make sure we didn't just repeat part of it
                break;
            }
        }
        //let digits = (1.0f64/(n as f64)).to_string().as_bytes().iter().skip(2).map(|x| (x-48));
        // for d in digits {
        //     print!("{}", d);
        // }
    }
    d as u64;
    0
}
pub fn p27() -> u64 {
    let mut len = 1;
    let mut prod = 1i64;
    for a in (-999i64..1000) {
        for b in (-999i64..1000) {
            let l = (0i64..).take_while(|&n| is_prime(abs(n*n + a*n + b) as u64)).count();
            if l > len {
                len = l;
                prod = a*b;
            }
        }
    }
    abs(prod) as u64
}
pub fn p28() -> u64 {
    0
}
pub fn p29() -> u64 {
    let mut nums: Vec<_> = (2..101).flat_map(|a| (2..101).map(|b| pow(BigUint::from_slice(&[a]), b)).collect::<Vec<_>>()).collect::<Vec<_>>();
    nums.sort();
    nums.dedup();
    nums.len() as u64
}
pub fn p30() -> u64 {
    (10..500000).filter(|&n| {
        let ndigits = (1..).take_while(|&x| n / 10.pow(x) > 0).last().unwrap() + 1;
        (1..ndigits+1).map(|i| {
            let digit = n % 10.pow(i) / 10.pow(i-1);
            digit.pow(5)
        }).sum::<u64>() == n
    }).sum::<u64>()
}
