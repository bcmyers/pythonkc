extern crate rusty_primes;

use rusty_primes::{primes, primes_magic, primes_multi};

#[test]
fn test_ffi() {
    assert_eq!(primes(0), 0);
    assert_eq!(primes(1), 0);
    assert_eq!(primes(2), 1);
    assert_eq!(primes(100_000), 9_592);;

    assert_eq!(primes_magic(0), 0);
    assert_eq!(primes_magic(1), 0);
    assert_eq!(primes_magic(2), 1);
    assert_eq!(primes_magic(100_000), 9_592);

    assert_eq!(primes_multi(0, 3), 0);
    assert_eq!(primes_multi(1, 3), 0);
    assert_eq!(primes_multi(2, 3), 1);
    assert_eq!(primes_multi(100_000, 3), 9_592);
}
