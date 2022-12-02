#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0010_952914.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  How do I make a flat list out of a list of lists?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

import itertools

# Performance Analysis
from timeit import repeat
from itertools import chain
from functools import reduce

from typing import Iterable

# from collections import Iterable                            # < py38

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


class Answer1:
    '''
    Description : Answer1
    '''

    @staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer1::code1')
        l = [[1, 2, 3], [4, 5, 6], [7], [8, 9]]
        flat_list = [item for sublist in l for item in sublist]
        print(flat_list)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')
        data = [[1, 2, 3], [4, 5, 6], [7], [8, 9]] * 10
        t = min(repeat(lambda: [item for sublist in data for item in sublist]))
        print(t)
        t = min(repeat(lambda: sum(data, [])))
        print(t)
        t = min(repeat(lambda: reduce(lambda x, y: x + y, data)))
        print(t)

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer1::code3')

    @staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_RED}{LINE}{ENDCOLOR}')
        Answer1.code1()
        Answer1.code2()
        Answer1.code3()
        print(f'{FOREGROUND_RED}{LINE}{ENDCOLOR}')


class Answer2:
    '''
    Description : Answer2
    '''

    @staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer2::code1')
        list2d = [[1, 2, 3], [4, 5, 6], [7], [8, 9]]
        merged = list(itertools.chain(*list2d))
        print(merged)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer2::code2')
        list2d = [[1, 2, 3], [4, 5, 6], [7], [8, 9]]
        merged = list(itertools.chain.from_iterable(list2d))
        print(merged)

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer2::code3')
        data = [[1, 2, 3], [4, 5, 6], [7], [8, 9]] * 10
        t = min(repeat(lambda: list(itertools.chain.from_iterable(data))))
        print(t)
        t = min(repeat(lambda: [item for sublist in data for item in sublist]))
        print(t)
        t = min(repeat(lambda: sum(data, [])))
        print(t)
        t = min(repeat(lambda: reduce(lambda x, y: x + y, data)))
        print(t)

    @staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_GREEN}{LINE}{ENDCOLOR}')
        Answer2.code1()
        Answer2.code2()
        Answer2.code3()
        print(f'{FOREGROUND_GREEN}{LINE}{ENDCOLOR}')


def flatten(items):
    """Yield items from any nested iterable; see Reference."""
    for x in items:
        if isinstance(x, Iterable) and not isinstance(x, (str, bytes)):
            for sub_x in flatten(x):
                yield sub_x
        else:
            yield x


class Answer3:
    '''
    Description : Answer3
    '''

    @staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer3::code1')
        simple = [[1, 2, 3], [4, 5, 6], [7], [8, 9]]
        list(flatten(simple))
        # [1, 2, 3, 4, 5, 6, 7, 8, 9]

        complicated = [[1, [2]], (3, 4, {5, 6}, 7), 8, "9"]  # numbers, strs, nested & mixed
        list(flatten(complicated))
        # [1, 2, 3, 4, 5, 6, 7, 8, '9']

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer3::code2')

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer3::code3')

    @staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_BLUE}{LINE}{ENDCOLOR}')
        Answer3.code1()
        Answer3.code2()
        Answer3.code3()
        print(f'{FOREGROUND_BLUE}{LINE}{ENDCOLOR}')


class Answer4:
    '''
    Description : Answer4
    '''

    @staticmethod
    def code1():
        '''
        Description : code1
        '''
        print('Answer4::code1')

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer4::code2')

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer4::code3')

    @staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_YELLOW}{LINE}{ENDCOLOR}')
        Answer4.code1()
        Answer4.code2()
        Answer4.code3()
        print(f'{FOREGROUND_YELLOW}{LINE}{ENDCOLOR}')


def verify():
    '''
    Description : verify
    '''
    # Answer1.verify()
    # Answer2.verify()
    # Answer3.verify()
    # Answer4.verify()
