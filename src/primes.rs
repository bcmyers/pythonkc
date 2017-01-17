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

pub fn no_of_primes_magic(bound: usize) -> usize {
    (0..bound + 1)
        .collect::<Vec<usize>>()
        .par_iter()
        .filter(|&x| is_prime(*x))
        .count()
}

pub fn no_of_primes_multi(bound: usize, nprocs: usize) -> usize {
    let mut threads = Vec::new();
    for i in 0..nprocs {
        let chunk = (i..bound + 1).step_by(nprocs);
        let thread = thread::spawn(move || {
            let mut v: Vec<usize> = Vec::new();
            for j in chunk {
                let u: usize = match is_prime(j) {
                    false => 0,
                    true => 1,
                };
                v.push(u);
            }
            v.iter().sum()
        });
        threads.push(thread);
    }

    let mut results: Vec<usize> = Vec::new();
    for thread in threads {
        let result = thread.join().ok().expect("Could not join a process!");
        results.push(result);
    }
    results.iter().sum()
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
        assert_eq!(no_of_primes(2), 1);
        assert_eq!(no_of_primes(100_000), 9_592);
    }

    #[test]
    fn test_no_of_primes_magic() {
        assert_eq!(no_of_primes_magic(0), 0);
        assert_eq!(no_of_primes_magic(1), 0);
        assert_eq!(no_of_primes_magic(2), 1);
        assert_eq!(no_of_primes_magic(100_000), 9_592);
    }

    #[test]
    fn test_no_of_primes_multi() {
        assert_eq!(no_of_primes_multi(0, 3), 0);
        assert_eq!(no_of_primes_multi(1, 3), 0);
        assert_eq!(no_of_primes_multi(2, 3), 1);
        assert_eq!(no_of_primes_multi(100_000, 3), 9_592);
    }
}
