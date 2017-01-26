extern crate rusty_primes;

use rusty_primes::core;
use rusty_primes::ffi1;

#[test]
fn test_core() {
    assert_eq!(core::primes(0), 0);
    assert_eq!(core::primes(1), 0);
    assert_eq!(core::primes(2), 1);
    assert_eq!(core::primes(100_000), 9_592);;

    assert_eq!(core::primes_magic(0), 0);
    assert_eq!(core::primes_magic(1), 0);
    assert_eq!(core::primes_magic(2), 1);
    assert_eq!(core::primes_magic(100_000), 9_592);

    assert_eq!(core::primes_multi(0, 3), 0);
    assert_eq!(core::primes_multi(1, 3), 0);
    assert_eq!(core::primes_multi(2, 3), 1);
    assert_eq!(core::primes_multi(100_000, 3), 9_592);
}


#[test]
fn test_ffi1() {
    assert_eq!(ffi1::primes(0), 0);
    assert_eq!(ffi1::primes(1), 0);
    assert_eq!(ffi1::primes(2), 1);
    assert_eq!(ffi1::primes(100_000), 9_592);

    assert_eq!(ffi1::primes_magic(0), 0);
    assert_eq!(ffi1::primes_magic(1), 0);
    assert_eq!(ffi1::primes_magic(2), 1);
    assert_eq!(ffi1::primes_magic(100_000), 9_592);

    assert_eq!(ffi1::primes_multi(0, 3), 0);
    assert_eq!(ffi1::primes_multi(1, 3), 0);
    assert_eq!(ffi1::primes_multi(2, 3), 1);
    assert_eq!(ffi1::primes_multi(100_000, 3), 9_592);
}
