#!/usr/bin/env python3.10
"""Map and Lambda
Jerod Gawne, 2022.03.23 <https://github.com/jerodg/hackerrank>"""

cube = lambda x: x ** 3


def fibonacci(n):
    ls = [0, 1]
    for i in range(2, n):
        ls.append(ls[i - 2] + ls[i - 1])

    return ls[0:n]


if __name__ == '__main__':
    n = int(input())
    print(list(map(cube, fibonacci(n))))