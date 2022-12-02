import random
import bisect
import matplotlib.pyplot as plt
import math
import timeit
import itertools

def wrapper(func, *args, **kwargs):
    " Use to produced 0 argument function for call it"
    # Reference https://www.pythoncentral.io/time-a-python-function/
    def wrapped():
        return func(*args, **kwargs)
    return wrapped

def method_in(a,b,c):
    for i,x in enumerate(a):
        if x in b:
            c[i] = b.index(x)
        else:
            c[i] = -1
    return c

def method_try(a,b,c):
    for i, x in enumerate(a):
        try:
            c[i] = b.index(x)
        except ValueError:
            c[i] = -1

def method_set_in(a,b,c):
    s = set(b)
    for i,x in enumerate(a):
        if x in s:
            c[i] = b.index(x)
        else:
            c[i] = -1
    return c

def method_bisect(a,b,c):
    " Finds indexes using bisection "

    # Create a sorted b with its index
    bsorted = sorted([(x, i) for i, x in enumerate(b)], key = lambda t: t[0])

    for i,x in enumerate(a):
        index = bisect.bisect_left(bsorted,(x, ))
        c[i] = -1
        if index < len(a):
            if x == bsorted[index][0]:
                c[i] = bsorted[index][1]  # index in the b array

    return c

def method_reverse_lookup(a, b, c):
    reverse_lookup = {x:i for i, x in enumerate(b)}
    for i, x in enumerate(a):
        c[i] = reverse_lookup.get(x, -1)
    return c

def profile():
    Nls = [x for x in range(1000,20000,1000)]
    number_iterations = 10
    methods = [method_in, method_try, method_set_in, method_bisect, method_reverse_lookup]
    time_methods = [[] for _ in range(len(methods))]

    for N in Nls:
        a = [x for x in range(0,N)]
        random.shuffle(a)
        b = [x for x in range(0,N)]
        random.shuffle(b)
        c = [0 for x in range(0,N)]

        for i, func in enumerate(methods):
            wrapped = wrapper(func, a, b, c)
            time_methods[i].append(math.log(timeit.timeit(wrapped, number=number_iterations)))

    markers = itertools.cycle(('o', '+', '.', '>', '2'))
    colors = itertools.cycle(('r', 'b', 'g', 'y', 'c'))
    labels = itertools.cycle(('in', 'try', 'set', 'bisect', 'reverse'))

    for i in range(len(time_methods)):
        plt.plot(Nls,time_methods[i],marker = next(markers),color=next(colors),linestyle='-',label=next(labels))

    plt.xlabel('list size', fontsize=18)
    plt.ylabel('log(time)', fontsize=18)
    plt.legend(loc = 'upper left')
    plt.show()

profile()
