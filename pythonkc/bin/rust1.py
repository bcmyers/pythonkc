# Not working

import sys

from rustypy import RustyModule

from pythonkc import (
    primes,
    primes_magic,
    primes_multi,
    Rust,
)
from pythonkc.timeit import timeit
from pythonkc.util import parse_args


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
