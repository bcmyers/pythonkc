#![allow(dead_code)]
// TODO benchmarks
// TODO docs
// TODO passing complex types

#![feature(step_by)]

#[macro_use]
extern crate cpython;
extern crate libc;
extern crate rayon;

// mod ffi;
mod ffi2;
pub mod general;
mod objects;
mod primes;

// pub use ffi::*;
pub use ffi2::*;
