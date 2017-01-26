use libc;

use core;


#[no_mangle]
pub extern "C" fn primes(bound: libc::c_uint) -> libc::c_uint {
    core::primes(bound)
}

#[no_mangle]
pub extern "C" fn primes_magic(bound: libc::c_uint) -> libc::c_uint {
    core::primes_magic(bound)
}

#[no_mangle]
pub extern "C" fn primes_multi(bound: libc::c_uint, nprocs: libc::c_uint) -> libc::c_uint {
    core::primes_multi(bound, nprocs)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        assert_eq!(primes(0), 0);
        assert_eq!(primes(1), 0);
        assert_eq!(primes(2), 1);
        assert_eq!(primes(100_000), 9_592);
    }

    #[test]
    fn test_primes_magic() {
        assert_eq!(primes_magic(0), 0);
        assert_eq!(primes_magic(1), 0);
        assert_eq!(primes_magic(2), 1);
        assert_eq!(primes_magic(100_000), 9_592);
    }

    #[test]
    fn test_primes_multi() {
        assert_eq!(primes_multi(0, 3), 0);
        assert_eq!(primes_multi(1, 3), 0);
        assert_eq!(primes_multi(2, 3), 1);
        assert_eq!(primes_multi(100_000, 3), 9_592);
    }

}
