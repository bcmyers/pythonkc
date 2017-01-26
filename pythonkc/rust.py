from rustypy import RustyBase


class Rust(RustyBase):
    def primes(self, bound: int) -> int:
        return self.lib.primes(bound)

    def primes_magic(self, bound: int) -> int:
        return self.lib.primes_magic(bound)

    def primes_multi(self, bound: int, nprocs: int) -> int:
        return self.lib.primes_multi(bound, nprocs)
