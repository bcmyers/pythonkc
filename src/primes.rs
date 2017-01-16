use rayon::prelude::*;

#[inline(always)]
fn is_prime(n: usize) -> bool {
    if (n == 2) | (n == 3) {
        return true;
    } else if (n % 2 == 0) | (n < 2) {
        return false;
    }
    let upper_bound = (n as f64).sqrt() as usize;
    let range = (3...upper_bound).filter(|x| x % 2 == 1);
    for i in range {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn no_of_primes(bound: usize) -> usize {
    (0...bound).filter(|&x| is_prime(x)).count()
}

pub fn no_of_primes_multi(bound: usize) -> usize {
    (0...bound)
        .collect::<Vec<usize>>()
        .par_iter()
        .filter(|&x| is_prime(*x))
        .count()
}
