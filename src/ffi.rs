use std::ffi::CStr;
use std::str;

use libc;

use primes;

#[no_mangle]
pub extern "C" fn no_of_primes(bound: libc::uint64_t) -> libc::uint64_t {
    primes::no_of_primes(bound as usize) as u64
}

#[no_mangle]
pub extern "C" fn no_of_primes_multi(bound: libc::uint64_t) -> libc::uint64_t {
    primes::no_of_primes_multi(bound as usize) as u64
}

#[no_mangle]
pub extern "C" fn rust_none_none() {
    println!("💩");
}

#[no_mangle]
pub extern "C" fn rust_int_none(arg: libc::c_int) {
    println!("{}", arg);
}

#[no_mangle]
pub unsafe extern "C" fn rust_string_none(arg: *const libc::c_char) {
    let c_str: &CStr = CStr::from_ptr(arg);
    let buf: &[u8] = c_str.to_bytes();
    let str_slice = str::from_utf8(buf).unwrap();
    println!("{}", str_slice);
}

#[no_mangle]
pub extern "C" fn rust_none_int() -> libc::c_int {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

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
