#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0102_209840.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  How can I make a dictionary (dict) from separate lists of keys and values?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

import numpy
import timeit

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
        keys = ['a', 'b', 'c']
        values = [1, 2, 3]
        dictionary = dict(zip(keys, values))
        print(dictionary)  # {'a': 1, 'b': 2, 'c': 3}
        assert {'a': 1, 'b': 2, 'c': 3} == dictionary

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')

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

        keys = ('name', 'age', 'food')
        values = ('Monty', 42, 'spam')

        print(min(timeit.repeat(lambda: dict(zip(keys, values)))))
        print( min(timeit.repeat(lambda: {k: v for k, v in zip(keys, values)})))
        print( min(timeit.repeat(lambda: {keys[i]: values[i] for i in range(len(keys))})))
        print( min(timeit.repeat(lambda: dict([(k, v) for k, v in zip(keys, values)]))))
        print( min(timeit.repeat(lambda: dict((k, v) for k, v in zip(keys, values)))))


        l1 = list(numpy.random.random(100))
        l2 = list(numpy.random.random(100))
        print(min(timeit.repeat(lambda: {k: v for k, v in zip(l1, l2)})))
        print( min(timeit.repeat(lambda: dict(zip(l1, l2)))))


    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer2::code2')

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer2::code3')

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
