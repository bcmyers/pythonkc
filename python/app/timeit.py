import time
from typing import Callable, Dict


def timeit(func: Callable[..., int], args: Dict[str, int]) -> int:
    t0 = time.time()
    func(**args)
    elapsed_time = time.time() - t0
    return elapsed_time
