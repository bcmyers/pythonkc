import psutil

DEFAULT_BOUND = 2_000_000
DEFAULT_NPROCS = psutil.cpu_count(logical=False) + 1
