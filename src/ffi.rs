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
pub extern "C" fn rust_string_none(arg: *const libc::c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(arg) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice = str::from_utf8(buf).unwrap();
    println!("{}", str_slice);
}

#[no_mangle]
pub extern "C" fn rust_none_int() -> libc::c_int {
    42
}
