import unittest

import rusty_primes


class TestRustyPrimes(unittest.TestCase):

    def test_primes(self):
        self.assertEqual(rusty_primes.primes(0), 0)
        self.assertEqual(rusty_primes.primes(1), 0)
        self.assertEqual(rusty_primes.primes(2), 1)
        self.assertEqual(rusty_primes.primes(100_000), 9_592)

    def test_no_of_primes_magic(self):
        self.assertEqual(rusty_primes.primes_magic(0), 0)
        self.assertEqual(rusty_primes.primes_magic(1), 0)
        self.assertEqual(rusty_primes.primes_magic(2), 1)
        self.assertEqual(rusty_primes.primes_magic(100_000), 9_592)

    def test_no_of_primes_multi(self):
        self.assertEqual(rusty_primes.primes_multi(0, 3), 0)
        self.assertEqual(rusty_primes.primes_multi(1, 3), 0)
        self.assertEqual(rusty_primes.primes_multi(2, 3), 1)
        self.assertEqual(rusty_primes.primes_multi(100_000, 3), 9_592)


if __name__ == '__main__':
    unittest.main()
