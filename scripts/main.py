import argparse
import os
import sys

import psutil

from pythonkc import (
    no_of_primes,
    no_of_primes_magic,
    no_of_primes_multi,
)
from pythonkc.rust import Rust, RUST_DIR
from pythonkc.timeit import timeit

DEFAULT_BOUND = 2_000_000
DEFAULT_NPROCS = psutil.cpu_count(logical=False) + 1


def parse_args(args):
    parser = argparse.ArgumentParser(
        description=(
            'Calculates the numer of prime numbers below a given bound.'
        )
    )
    parser.add_argument(
        'bound',
        default=DEFAULT_BOUND,
        help=(
            'an integer specifying the bound; '
            'default is {:,d}'
        ).format(DEFAULT_BOUND),
        nargs='?',
        type=int,
    )
    parser.add_argument(
        '--nprocs',
        default=DEFAULT_NPROCS,
        help=(
            'an integer specifying the number of processes to launch; '
            'default is the number of physical cores on your machine + 1'
        ),
        type=int,
    )
    return parser.parse_args(args)


def main():
    args = parse_args(sys.argv[1:])
    print('\nHow many prime numbers are there below {:,d}?'.format(args.bound))
    print('\nPython:')
    t1 = timeit(no_of_primes, {'bound': args.bound})
    print('Single process took {:.3f} seconds.'.format(t1))
    t2 = timeit(no_of_primes_magic, {'bound': args.bound})
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t2))
    t3 = timeit(no_of_primes_multi, {'bound': args.bound, 'nprocs': args.nprocs})  # noqa: E501
    print("'Manual' multiprocessing took {:.3f} seconds.".format(t3))
    rust = Rust()
    print('\nRust called from Python:')
    t4 = timeit(rust.no_of_primes, {'bound': args.bound})
    print('Single process took {:.3f} seconds.'.format(t4))
    t5 = timeit(rust.no_of_primes_magic, {'bound': args.bound})
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t5))
    t6 = timeit(rust.no_of_primes_multi, {'bound': args.bound, 'nprocs': 10})
    print("'Manual' multiprocessing took {:.3f} seconds.\n".format(t6))
    os.system('{} {}'.format(os.path.join(RUST_DIR, 'primes'), args.bound))


if __name__ == '__main__':
    main()
