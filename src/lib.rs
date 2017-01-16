// TODO benchmarks
// TODO docs
// TODO passing complex types

extern crate libc;
extern crate num_cpus;
extern crate rayon;

mod ffi;
pub mod general;
pub mod primes;

pub use ffi::*;
