use num::traits::PrimInt;
use funs::{ndigits};
use primal::{is_prime, Primes};

pub fn p31() -> u64 {
    let mut num_ways = 0;
    let mut total;
    for p200 in (0..201).step_by(200) {
        total = p200;
        for p100 in (0..201 - total).step_by(100) {
            total = p200 + p100;
            for p50 in (0..201 - total).step_by(50) {
                total = p200 + p100 + p50;
                for p20 in (0..201 - total).step_by(20) {
                    total = p200 + p100 + p50 + p20;
                    for p10 in (0..201 - total).step_by(10) {
                        total = p200 + p100 + p50 + p20 + p10;
                        for p5 in (0..201 - total).step_by(5) {
                            total = p200 + p100 + p50 + p20 + p10 + p5;
                            for p2 in (0..201 - total).step_by(2) {
                                total = p200 + p100 + p50 + p20 + p10 + p5 + p2;
                                for p1 in (0..201 - total) {
                                    total = p200 + p100 + p50 + p20 + p10 + p5 + p2 + p1;
                                    if total == 200 {
                                        num_ways += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    num_ways
}
pub fn p32() -> u64 {
    0
}
pub fn p33() -> u64 {
    0
}
pub fn p34() -> u64 {
    fn fac(n: u64) -> u64 {
        (1..n+1).product()
    }
    (10..2540160).filter(|&n| {
        let ndigits = (1..).take_while(|&x| n / 10.pow(x) > 0).last().unwrap() + 1;
        (1..ndigits+1).rev().map(|i| n % 10.pow(i) / 10.pow(i-1)).map(|i| fac(i)).sum::<u64>() == n
    }).sum::<u64>()
}

pub fn p35() -> u64 {
    (2..1_000_000).filter(|&n| {
        let ndig = ndigits(n);
        (0..ndig).all(|i| {
            let low = 10.pow(i as u32);
            let high = 10.pow((ndig - i) as u32);
            let perm = (n%high)*low + n/high;
            is_prime(perm)
        })
    }).count() as u64
}
pub fn p36() -> u64 {
    (1..1_000_000).filter(|&n| n%10 != 0).filter(|&n| n%2 != 0).filter(|&n| {
        true
    }).sum::<u64>();
    0
}
pub fn p37() -> u64 {
    Primes::all().skip_while(|&n| n < 10).filter(|&n| {
        let p = n as u64;
        let ndig = ndigits(p);
        (0..ndig).all(|i| {
            is_prime(p % 10.pow(ndig-i)) && is_prime(p / 10.pow(i))
        })
    }).take(11).sum::<usize>() as u64
}
pub fn p38() -> u64 {
    0
}
pub fn p39() -> u64 {
    0
}
pub fn p40() -> u64 {
    0
}
