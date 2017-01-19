import os
import sys
from typing import Union

import cffi

from config import RUST_DIR


def library(library_name: str) -> str:
    prefix = {'win32': ''}.get(sys.platform, 'lib')
    extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
    return os.path.join(RUST_DIR, prefix + library_name + extension)


def header(library_name: str) -> str:
    path = os.path.join(RUST_DIR, 'lib' + library_name + '.h')
    with open(path, 'r') as f:
        header = f.read()
    return header


class Rust:

    __slots__ = ('_ffi', '_lib')

    def __init__(self):
        self._ffi = cffi.FFI()
        self._ffi.cdef(header('pythonkc'))
        self._lib = self._ffi.dlopen(library('pythonkc'))

    def no_of_primes(self, bound: int) -> int:
        return self._lib.no_of_primes(bound)

    def no_of_primes_magic(self, bound: int) -> int:
        return self._lib.no_of_primes_magic(bound)

    def no_of_primes_multi(self, bound: int, nprocs: int) -> int:
        return self._lib.no_of_primes_multi(bound, nprocs)

    def rust_none_none(self) -> None:
        self._lib.rust_none_none()

    def rust_int_none(self, arg: int) -> None:
        self._lib.rust_int_none(arg)

    def rust_string_none(self, arg: Union[str, bytes]) -> None:
        if not isinstance(arg, bytes):
            arg = arg.encode('utf-8')
        self._lib.rust_string_none(arg)

    def rust_none_int(self) -> int:
        return self._lib.rust_none_int()

    def rust_none_string(self) -> str:
        return self._ffi.string(
            self._lib.rust_none_string()).decode('utf-8')
