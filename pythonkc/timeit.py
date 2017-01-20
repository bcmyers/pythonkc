import time
from typing import Callable, Dict, Tuple


def timeit(func: Callable[..., int], args: Dict[str, int]) -> Tuple[float, int]:  # noqa: E50
    t0 = time.time()
    answer = func(**args)
    elapsed_time = time.time() - t0
    return (elapsed_time, answer)
