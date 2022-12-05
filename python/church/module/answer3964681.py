#!/usr/bin/python3
"""
docstring
"""
# https://peps.python.org/pep-0471/
import os
import pathlib
import timeit
import glob


def a():
    """docstring"""
    path = pathlib.Path().cwd()
    list_sqlite_files = [str(f) for f in path.glob("*.sqlite")]


def b():
    """docstring"""
    path = os.getcwd()
    list_sqlite_files = [f.path for f in os.scandir(path) if os.path.splitext(f)[1] == ".sqlite"]


def c():
    """docstring"""
    path = os.getcwd()
    list_sqlite_files = [os.path.join(path, f) for f in os.listdir(path) if f.endswith(".sqlite")]


def d():
    """docstring"""
    path = os.getcwd()
    os.chdir(path)
    list_sqlite_files = [os.path.join(path, f) for f in glob.glob("*.sqlite")]


def e():
    """docstring"""
    path = os.getcwd()
    list_sqlite_files = [os.path.join(path, f) for f in glob.glob1(str(path), "*.sqlite")]


def f():
    """docstring"""
    path = os.getcwd()
    list_sqlite_files = []
    for root, dirs, files in os.walk(path):
        for file in files:
            if file.endswith(".sqlite"):
                list_sqlite_files.append(os.path.join(root, file))
        break


print(timeit.timeit(a, number=1000))
print(timeit.timeit(b, number=1000))
print(timeit.timeit(c, number=1000))
print(timeit.timeit(d, number=1000))
print(timeit.timeit(e, number=1000))
print(timeit.timeit(f, number=1000))


"""
# Python 3.6.4
0.431
0.515
0.161
0.548
0.537
0.274
"""

