extern crate pythonkc;

use pythonkc::ffi;
use pythonkc::primes;

#[test]
fn test_ffi() {
    assert_eq!(ffi::no_of_primes(0), 0);
    assert_eq!(ffi::no_of_primes(1), 0);
    assert_eq!(ffi::no_of_primes(100_000), 9_592);;
    assert_eq!(ffi::no_of_primes_multi(0), 0);
    assert_eq!(ffi::no_of_primes_multi(1), 0);
    assert_eq!(ffi::no_of_primes_multi(100_000), 9_592);
}

#[test]
fn test_primes() {
    assert_eq!(primes::no_of_primes(0), 0);
    assert_eq!(primes::no_of_primes(1), 0);
    assert_eq!(primes::no_of_primes(100_000), 9_592);;
    assert_eq!(primes::no_of_primes_multi(0), 0);
    assert_eq!(primes::no_of_primes_multi(1), 0);
    assert_eq!(primes::no_of_primes_multi(100_000), 9_592);
}
