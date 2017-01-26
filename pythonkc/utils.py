import argparse

from pythonkc.config import DEFAULT_BOUND, DEFAULT_NPROCS


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
