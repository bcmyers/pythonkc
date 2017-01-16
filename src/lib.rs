#![feature(inclusive_range_syntax)]

extern crate libc;
extern crate rayon;

mod ffi;
pub mod general;
pub mod primes;

pub use self::ffi::*;

// TODO unit tests
