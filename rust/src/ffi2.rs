use cpython::{PyResult, Python};

use core;

py_module_initializer!(rusty_primes,
                       initrusty_primes,
                       PyInit_rusty_primes,
                       |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust."));
    try!(m.add(py, "primes", py_fn!(py, primes_py(n: u32))));
    try!(m.add(py, "primes_magic", py_fn!(py, primes_magic_py(bound: u32))));
    try!(m.add(py,
               "primes_multi",
               py_fn!(py, primes_multi_py(bound: u32, nprocs: u32))));
    Ok(())
});

fn primes_py(_: Python, bound: u32) -> PyResult<u32> {
    Ok(core::primes(bound))
}

fn primes_magic_py(_: Python, bound: u32) -> PyResult<u32> {
    Ok(core::primes_magic(bound))
}

fn primes_multi_py(_: Python, bound: u32, nprocs: u32) -> PyResult<u32> {
    Ok(core::primes_multi(bound, nprocs))
}
