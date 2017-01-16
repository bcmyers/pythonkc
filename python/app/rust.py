import os
import sys
from typing import Union

import cffi

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
LIBRARY_DIR = os.path.join(
    os.path.dirname(os.path.dirname(BASE_DIR)),
    'target', 'release')


def extension():
    extensions = {
        'darwin': '.dylib',
        'win32': '.dll',
        'cygwin': '.dll'}
    return extensions.get(sys.platform, '.so')


class Rust:

    __slots__ = ('_ffi', '_lib')

    def __init__(self):
        self._ffi = cffi.FFI()
        self._ffi.cdef(
            'void rust_none_none();' +
            'void rust_int_none(int);' +
            'void rust_string_none(char*);' +
            'int rust_none_int();' +
            'char* rust_none_string();' +
            'int no_of_primes(int);' +
            'int no_of_primes_multi(int);')
        self._lib = self._ffi.dlopen(
            os.path.join(LIBRARY_DIR, 'libpythonkc' + extension()))

    def no_of_primes(self, bound: int) -> int:
        return self._lib.no_of_primes(bound)

    def no_of_primes_multi(self, bound: int) -> int:
        return self._lib.no_of_primes_multi(bound)

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
