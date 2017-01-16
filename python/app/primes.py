import multiprocessing
from typing import Iterator, Optional  # noqa


def is_prime(n: int) -> bool:
    if n == 2 or n == 3:
        return True
    if n % 2 == 0 or n < 2:
        return False
    for i in range(3, int(n ** 0.5) + 1, 2):
        if n % i == 0:
            return False
    return True


def no_of_primes(bound: int) -> int:
    return len([n for n in range(0, bound + 1) if is_prime(n)])


def no_of_primes_multi(bound: int, nprocs: int) -> int:

    def work(chunk: Iterator[int], queue: 'multiprocessing.Queue') -> None:
        result = len([n for n in chunk if is_prime(n)])
        queue.put(result)

    def generate_chunks(iterator, items_per_chunk):
        for i in range(0, len(iterator), items_per_chunk):
            yield iterator[i:i + items_per_chunk]

    chunks = list(generate_chunks(range(0, bound + 1), bound // nprocs + 1))

    queue = multiprocessing.Queue()

    for i in range(nprocs):
        p = multiprocessing.Process(
                target=work,
                args=(chunks[i], queue))
        p.start()

    answer = 0
    for i in range(nprocs):
        answer += queue.get()

    for i in range(nprocs):
        p.join()

    return answer
