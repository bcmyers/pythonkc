import os

import psutil

from primes import no_of_primes, no_of_primes_multi
from rust import Rust, LIBRARY_DIR
from timeit import timeit

BOUND = 2_000_000
NPROCS = psutil.cpu_count(logical=False) + 1


def main():

    print('\nProcessing {:,d} numbers'.format(BOUND))

    a = timeit(no_of_primes, {'bound': BOUND})
    b = timeit(no_of_primes_multi, {'bound': BOUND, 'nprocs': NPROCS})

    print('\nPython:')
    print('Single process took {:.3f} seconds.'.format(a))
    print('Three processes took {:.3f} seconds.'.format(b))

    rust = Rust()

    c = timeit(rust.no_of_primes, {'bound': BOUND})
    d = timeit(rust.no_of_primes_multi, {'bound': BOUND})

    print('\nRust called from Python:')
    print('Single process took {:.3f} seconds.'.format(c))
    print('Multiple processes took {:.3f} seconds.'.format(d))

    os.system('{}/main'.format(LIBRARY_DIR))


if __name__ == '__main__':
    main()
