import sys
import time

from pythonkc import (
    primes,
    primes_magic,
    primes_multi,
)
from pythonkc.util import parse_args
import rusty_primes


def main():
    args = parse_args(sys.argv[1:])

    print('\nHow many prime numbers are there below {:,d}?'.format(args.bound))

    print('\nPython:')

    t0 = time.time()
    np = primes(args.bound)
    t1 = time.time() - t0
    print('Single process took {:.3f} seconds.'.format(t1))

    t0 = time.time()
    np = primes_magic(args.bound)
    t2 = time.time() - t0
    print("'Magic' multiprocessing took {:.3f} seconds.".format(t2))

    t0 = time.time()
    np = primes_multi(args.bound, args.nprocs)
    t3 = time.time() - t0
    print("'Manual' multiprocessing took {:.3f} seconds.".format(t3))

    print('\nRust called from Python:')

    t0 = time.time()
    np = rusty_primes.primes(args.bound)
    t4 = time.time() - t0
    print((
        'Single process took {:.3f} seconds ({:.1f}x faster).'
        .format(t4, t1/t4)
    ))

    t0 = time.time()
    np = rusty_primes.primes_magic(args.bound)
    t5 = time.time() - t0
    print((
        "'Magic' multiprocessing took {:.3f} seconds ({:.1f}x faster)."
        .format(t5, t2/t5)
    ))

    t0 = time.time()
    np = rusty_primes.primes_multi(args.bound, args.nprocs)
    t6 = time.time() - t0
    print((
        "'Manual' multiprocessing took {:.3f} seconds ({:.1f}x faster).\n"
        .format(t6, t3/t6)
    ))

    percentage = np / args.bound * 100
    print('In case you were curious...')
    print((
        'There are {:,d} primes below {:,d} ({:.1f}%)\n'
        .format(np, args.bound, percentage)
    ))


if __name__ == '__main__':
    main()
