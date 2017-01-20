import argparse
import sys

import psutil
from rustypy import RustyModule

from pythonkc import (
    primes,
    primes_magic,
    primes_multi,
    Rust,
)
from pythonkc.timeit import timeit

DEFAULT_BOUND = 2_000_000
DEFAULT_NPROCS = psutil.cpu_count(logical=False) + 1


def parse_args(args: str) -> argparse.Namespace:
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
    t1, np = timeit(primes, {'bound': args.bound})
    print('Single process took {:.3f} seconds.'.format(t1))
    t2, np = timeit(primes_magic, {'bound': args.bound})
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t2))
    t3, np = timeit(primes_multi, {'bound': args.bound, 'nprocs': args.nprocs})
    print("'Manual' multiprocessing took {:.3f} seconds.".format(t3))
    rusty_module = RustyModule('pythonkc')
    rust = Rust(rusty_module)
    print('\nRust called from Python:')
    t4, np = timeit(rust.primes, {'bound': args.bound})
    print('Single process took {:.3f} seconds.'.format(t4))
    t5, np = timeit(rust.primes_magic, {'bound': args.bound})
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t5))
    t6, np = timeit(rust.primes_multi, {'bound': args.bound, 'nprocs': 10})
    print("'Manual' multiprocessing took {:.3f} seconds.\n".format(t6))
    print('In case you were curious...')
    percentage = np / args.bound * 100
    print((
        'There are {:,d} primes below {:,d} ({:.1f}%)\n'
        .format(np, args.bound, percentage)
    ))


if __name__ == '__main__':
    main()
