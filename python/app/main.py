import os

import psutil

from primes import no_of_primes, no_of_primes_magic, no_of_primes_multi
from rust import Rust, TARGET_DIR
from timeit import timeit

BOUND = 5_000_000
try:
    NPROCS = (psutil.cpu_count(logical=False) + 1) * 2
except:
    NPROCS = 6


def main():

    print('\nHow many prime numbers are there below {:,d}?'.format(BOUND))

    print('\nPython:')
    t1 = timeit(no_of_primes, {'bound': BOUND})
    print('Single process took {:.3f} seconds.'.format(t1))
    t2 = timeit(no_of_primes_magic, {'bound': BOUND})
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t2))
    t3 = timeit(no_of_primes_multi, {'bound': BOUND, 'nprocs': NPROCS})
    print("'Manual' multiprocessing took {:.3f} seconds.".format(t3))

    rust = Rust()
    print('\nRust called from Python:')
    t4 = timeit(rust.no_of_primes, {'bound': BOUND})
    print('Single process took {:.3f} seconds.'.format(t4))
    t5 = timeit(rust.no_of_primes_magic, {'bound': BOUND})
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t5))
    t6 = timeit(rust.no_of_primes_multi, {'bound': BOUND, 'nprocs': 10})
    print("'Manual' multiprocessing took {:.3f} seconds.".format(t6))

    os.system('{} {}'.format(
        os.path.join(TARGET_DIR, 'release', 'primes'),
        BOUND))


if __name__ == '__main__':
    main()
