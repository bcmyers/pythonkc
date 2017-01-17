// TODO benchmarks
// TODO docs
// TODO passing complex types

#![feature(step_by)]

extern crate libc;
extern crate rayon;

mod ffi;
pub mod general;
pub mod primes;

pub use ffi::*;
