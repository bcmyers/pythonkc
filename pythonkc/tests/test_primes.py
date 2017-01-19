import unittest

from pythonkc.primes import (
    is_prime,
    no_of_primes,
    no_of_primes_magic,
    no_of_primes_multi,
)


class TestPrimes(unittest.TestCase):

    def is_prime(self):
        self.assertTrue(is_prime(101))
        self.assertTrue(not is_prime(25))

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


if __name__ == '__main__':
    unittest.main()
