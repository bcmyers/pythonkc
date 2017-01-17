import unittest

from .primes import no_of_primes, no_of_primes_magic, no_of_primes_multi
from .rust import Rust


class TestPrimes(unittest.TestCase):

    def test_no_of_primes(self):
        self.assertEqual(no_of_primes(0), 0)
        self.assertEqual(no_of_primes(1), 0)
        self.assertEqual(no_of_primes(2), 1)
        self.assertEqual(no_of_primes(100_000), 9_592)

    def test_no_of_primes_magic(self):
        self.assertEqual(no_of_primes_magic(0), 0)
        self.assertEqual(no_of_primes_magic(1), 0)
        self.assertEqual(no_of_primes_magic(2), 1)
        self.assertEqual(no_of_primes_magic(100_000), 9_592)

    def test_no_of_primes_multi(self):
        self.assertEqual(no_of_primes_multi(0, 3), 0)
        self.assertEqual(no_of_primes_multi(1, 3), 0)
        self.assertEqual(no_of_primes_multi(2, 3), 1)
        self.assertEqual(no_of_primes_multi(100_000, 3), 9_592)


class TestRust(unittest.TestCase):

    def test_no_of_primes(self):
        rust = Rust()
        self.assertEqual(rust.no_of_primes(0), 0)
        self.assertEqual(rust.no_of_primes(1), 0)
        self.assertEqual(rust.no_of_primes(2), 1)
        self.assertEqual(rust.no_of_primes(100_000), 9_592)

    def test_no_of_primes_magic(self):
        rust = Rust()
        self.assertEqual(rust.no_of_primes_magic(0), 0)
        self.assertEqual(rust.no_of_primes_magic(1), 0)
        self.assertEqual(rust.no_of_primes_magic(2), 1)
        self.assertEqual(rust.no_of_primes_magic(100_000), 9_592)

    def test_no_of_primes_multi(self):
        rust = Rust()
        self.assertEqual(rust.no_of_primes_multi(0, 3), 0)
        self.assertEqual(rust.no_of_primes_multi(1, 3), 0)
        self.assertEqual(rust.no_of_primes_multi(2, 3), 1)
        self.assertEqual(rust.no_of_primes_multi(100_000, 3), 9_592)


if __name__ == '__main__':
    unittest.main()
