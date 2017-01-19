import multiprocessing
from typing import Iterator, Optional  # noqa: F401


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
    nums = range(0, bound + 1)
    return len([n for n in nums if is_prime(n)])


def no_of_primes_magic(bound: int) -> int:
    nums = range(0, bound + 1)
    pool = multiprocessing.Pool()
    result = pool.map(is_prime, nums)
    return sum(result)


def no_of_primes_multi(bound: int, nprocs: int) -> int:
    nums = range(0, bound + 1)
    chunks = [nums[i::nprocs] for i in range(nprocs)]

    def worker(chunk: Iterator[int], queue: 'multiprocessing.Queue') -> None:
        result = len([n for n in chunk if is_prime(n)])
        queue.put(result)

    queue = multiprocessing.Queue()
    processes = [
        multiprocessing.Process(target=worker, args=(chunks[i], queue))
        for i in range(nprocs)
    ]
    for process in processes:
        process.start()
    for process in processes:
        process.join()
    answer = sum([queue.get() for process in processes])
    return answer
