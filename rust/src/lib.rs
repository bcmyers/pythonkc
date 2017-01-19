#![allow(dead_code)]
// TODO benchmarks
// TODO docs
// TODO passing complex types

#![feature(step_by)]

extern crate env_logger;
extern crate libc;
#[macro_use]
extern crate log;
extern crate rayon;

mod ffi;
pub mod general;
mod objects;
pub mod primes;

pub use ffi::*;
