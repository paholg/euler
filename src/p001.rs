use std::cmp::max;
use num::traits::{ToPrimitive, Float, PrimInt};
use num::integer::lcm;
use primal::Primes;

pub fn p1() -> u64 {
    // fold is because sum is unstable
    let s: u64 = (1..1000).filter(|x| x%3 == 0 || x%5 == 0).fold(0, |a, b| a + b);
    s
}

pub fn p2() -> u64 {
    let (mut a, mut b): (u32, u32) = (1, 2);
    let mut sum: u32 = 0;
    while b < 4000000 {
        if b % 2 == 0 {
            sum += b;
        }
        let temp = b;
        b = a + b;
        a = temp;
    }
    sum as u64
}

pub fn p3() -> u64 {
    let mut n = 600851475143;
    let mut m = 1;
    while m < n {
        let limit: usize = (n as f32).sqrt().round().to_usize().unwrap();
        let fac = Primes::all().take_while(|&x| x < limit).find(|x| n%x == 0).unwrap_or(n);
        n /= fac;
        m = max(m, fac);
    }
    m as u64
}
pub fn p4() -> u64 {
    let low = 100;
    let high = 999;
    let mut palindrome = low*low + 1; // minimum possible answer ... setting to this for fun
    for a in (low..high+1).rev() {
        if a*high < palindrome {
            // impossible to do better at this point.
            break
        }
        'bloop: for b in (a..high+1).rev() {
            let prod = a*b;
            let ndigits = (1..).take_while(|&x| prod / 10.pow(x) > 0).last().unwrap() + 1;
            for i in (1..ndigits + 1) {
                if (prod % 10.pow(i)) / 10.pow(i-1)  != (prod % 10.pow(ndigits+1-i)) / 10.pow(ndigits-i) {
                    continue 'bloop;
                }
            }
            palindrome = max(palindrome, prod);
        }
    }
    palindrome
}
pub fn p5() -> u64 {
    let num = (1u64..21).fold(1, |a, b| lcm(a, b));
    num as u64
}
pub fn p6() -> u64 {
    let n: u64 = 100;
    let sum_squares: u64 = (1..n+1).map(|x| x*x).sum();
    let sum: u64 = n*(n+1)/2;
    let square_sum: u64 = sum*sum;
    (square_sum - sum_squares) as u64
}
pub fn p7() -> u64 {
    let p = Primes::all().take(10001).last().unwrap();
    p as u64
}
pub fn p8() -> u64 {
    let n = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let mut nums = n.as_bytes().iter().map(|x| (x - 48) as u64).collect::<Vec<_>>();
    let mut prod: u64 = nums.iter().rev().take(13).product();
    while nums.pop() != None {
        prod = max(prod, nums.iter().rev().take(13).product());
    }
    prod as u64
}
pub fn p9() -> u64 {
    for c in (1..500) {
        for a in (1..c) {
            for b in (1..a) {
                if a*a + b*b == c*c {
                    if a + b + c == 1000 {
                        return a*b*c as u64;
                    }
                }
            }
        }
    }
    unreachable!()
}
pub fn p10() -> u64 {
    let limit = 2_000_000;
    let sum: usize = Primes::all().take_while(|&n| n < limit).sum();
    sum as u64
}
