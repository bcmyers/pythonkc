import unittest

from rustypy import RustyModule

from pythonkc.rust import Rust


class TestRust(unittest.TestCase):

    def test_no_of_primes(self):
        rusty_module = RustyModule('pythonkc')
        rust = Rust(rusty_module)
        self.assertEqual(rust.primes(0), 0)
        self.assertEqual(rust.primes(1), 0)
        self.assertEqual(rust.primes(2), 1)
        self.assertEqual(rust.primes(100_000), 9_592)

    def test_no_of_primes_magic(self):
        rusty_module = RustyModule('pythonkc')
        rust = Rust(rusty_module)
        self.assertEqual(rust.primes_magic(0), 0)
        self.assertEqual(rust.primes_magic(1), 0)
        self.assertEqual(rust.primes_magic(2), 1)
        self.assertEqual(rust.primes_magic(100_000), 9_592)

    def test_no_of_primes_multi(self):
        rusty_module = RustyModule('pythonkc')
        rust = Rust(rusty_module)
        self.assertEqual(rust.primes_multi(0, 3), 0)
        self.assertEqual(rust.primes_multi(1, 3), 0)
        self.assertEqual(rust.primes_multi(2, 3), 1)
        self.assertEqual(rust.primes_multi(100_000, 3), 9_592)


if __name__ == '__main__':
    unittest.main()
