#![allow(dead_code)]
// TODO benchmarks
// TODO docs
// TODO passing complex types

#![feature(step_by)]

#[macro_use]
extern crate cpython;
extern crate libc;
extern crate rayon;

pub mod core;
pub mod ffi1;
mod ffi2;
pub mod general;
mod objects;

pub use ffi1::*;
pub use ffi2::*;
