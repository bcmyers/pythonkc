import sys
from typing import Union

import cffi


def extension():
    extensions = {
        'darwin': '.dylib',
        'win32': '.dll',
        'cygwin': '.dll'}
    return extensions.get(sys.platform, '.so')


class Rust:

    __slots__ = ('_ffi', '_library')

    def __init__(self):
        self._ffi = cffi.FFI()
        self._ffi.cdef(
            'void rust_none_none();' +
            'void rust_int_none(int);' +
            'void rust_string_none(char*);' +
            'int rust_none_int();' +
            'char* rust_none_string();')
        self._lib = self._ffi.dlopen(
            'target/debug/libpythonkc' + extension())

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


if __name__ == '__main__':
    rust = Rust()
    rust.rust_none_none()
    rust.rust_int_none(1)
    rust.rust_string_none('Hello 💩')
    msg = rust.rust_none_int()
    print(msg)
    msg = rust.rust_none_string()
    print(msg)
