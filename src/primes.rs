use std::thread;

use rayon::prelude::*;

fn is_prime(n: usize) -> bool {
    if (n == 2) | (n == 3) {
        return true;
    } else if (n % 2 == 0) | (n < 2) {
        return false;
    }
    let upper_bound = (n as f64).sqrt() as usize;
    let range = (3..upper_bound + 1).filter(|x| x % 2 == 1);
    for i in range {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn no_of_primes(bound: usize) -> usize {
    (0..bound + 1).filter(|&x| is_prime(x)).count()
}

pub fn no_of_primes_multi(bound: usize) -> usize {
    (0..bound + 1)
        .collect::<Vec<usize>>()
        .par_iter()
        .filter(|&x| is_prime(*x))
        .count()
}

// TODO
pub fn process() {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let mut _x = 0;
                for _ in 0..5_000_001 {
                    _x += 1
                }
            })
        })
        .collect();
    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(101));
        assert!(is_prime(999_983));
        assert!(!is_prime(4));
        assert!(!is_prime(25));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_no_of_primes() {
        assert_eq!(no_of_primes(0), 0);
        assert_eq!(no_of_primes(1), 0);
        assert_eq!(no_of_primes(100_000), 9_592);
    }

    #[test]
    fn test_no_of_primes_multi() {
        assert_eq!(no_of_primes_multi(0), 0);
        assert_eq!(no_of_primes_multi(1), 0);
        assert_eq!(no_of_primes_multi(100_000), 9_592);
    }

}
