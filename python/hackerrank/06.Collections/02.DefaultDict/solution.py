#!/usr/bin/env python3.7
"""Default Dict: Jerod Gawne, 2019.02.25 <https://github.com/jerodg>"""

from collections import defaultdict
from sys import exc_info
from traceback import print_exception
from typing import NoReturn


def main() -> NoReturn:
    d = defaultdict(list)
    n, m = list(map(int, input().split()))

    for i in range(n):
        d[input()].append(i + 1)

    for i in range(m):
        print(' '.join(map(str, d[input()])) or -1)


if __name__ == '__main__':
    try:
        main()
    except Exception:
        print(print_exception(*exc_info()))
