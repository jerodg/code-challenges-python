#!/usr/bin/env python3.7
"""Strict Supersets: Jerod Gawne, 2019.02.18 <https://github.com/jerodg>"""

from sys import exc_info
from traceback import print_exception
from typing import NoReturn


def main() -> NoReturn:
    a = set(input().split())
    print(all(a > set(input().split()) for _ in range(int(input()))))


if __name__ == '__main__':
    try:
        main()
    except Exception:
        print(print_exception(*exc_info()))
