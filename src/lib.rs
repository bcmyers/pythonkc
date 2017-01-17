#![allow(dead_code)]
// TODO benchmarks
// TODO docs
// TODO passing complex types

#![feature(step_by)]

extern crate libc;
extern crate rayon;

mod ffi;
pub mod general;
mod objects;
pub mod primes;

pub use ffi::*;
