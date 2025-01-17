#!/usr/bin/env python3.7
# coding=utf-8
"""Jerod Gawne, 2018.07.24

https://github.com/jerodg/hackerrank
"""

import itertools
import sys
import traceback

if __name__ == '__main__':
    try:
        print(*((len(list(c)), int(k)) for k, c in itertools.groupby(input())))
    except Exception:
        print(traceback.print_exception(*sys.exc_info()))
