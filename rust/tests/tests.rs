extern crate pythonkc;

use pythonkc::{no_of_primes, no_of_primes_magic, no_of_primes_multi};
use pythonkc::primes;

#[test]
fn test_ffi() {
    assert_eq!(no_of_primes(0), 0);
    assert_eq!(no_of_primes(1), 0);
    assert_eq!(no_of_primes(2), 1);
    assert_eq!(no_of_primes(100_000), 9_592);;

    assert_eq!(no_of_primes_magic(0), 0);
    assert_eq!(no_of_primes_magic(1), 0);
    assert_eq!(no_of_primes_magic(2), 1);
    assert_eq!(no_of_primes_magic(100_000), 9_592);

    assert_eq!(no_of_primes_multi(0, 3), 0);
    assert_eq!(no_of_primes_multi(1, 3), 0);
    assert_eq!(no_of_primes_multi(2, 3), 1);
    assert_eq!(no_of_primes_multi(100_000, 3), 9_592);
}

#[test]
fn test_primes() {
    assert_eq!(primes::no_of_primes(0), 0);
    assert_eq!(primes::no_of_primes(1), 0);
    assert_eq!(primes::no_of_primes(2), 1);
    assert_eq!(primes::no_of_primes(100_000), 9_592);;

    assert_eq!(primes::no_of_primes_magic(0), 0);
    assert_eq!(primes::no_of_primes_magic(1), 0);
    assert_eq!(primes::no_of_primes_magic(2), 1);
    assert_eq!(primes::no_of_primes_magic(100_000), 9_592);

    assert_eq!(primes::no_of_primes_multi(0, 3), 0);
    assert_eq!(primes::no_of_primes_multi(1, 3), 0);
    assert_eq!(primes::no_of_primes_multi(2, 3), 1);
    assert_eq!(primes::no_of_primes_multi(100_000, 3), 9_592);
}