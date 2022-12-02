#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0035_252703.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  What is the difference between Python's list methods append and extend?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

import timeit

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


# Performance
def append(alist, iterable):
    '''
    Description :
    '''
    for item in iterable:
        alist.append(item)


def extend(alist, iterable):
    '''
    Description :
    '''
    alist.extend(iterable)


def perf1():
    '''
    Description :
    '''
    print(min(timeit.repeat(lambda: append([], "abcdefghijklmnopqrstuvwxyz"))))
    print(min(timeit.repeat(lambda: extend([], "abcdefghijklmnopqrstuvwxyz"))))


# Perfect answer, I just miss the timing of comparing adding only one element


def append_one(a_list, element):
    '''
    Description :
    '''
    a_list.append(element)


def extend_one(a_list, element):
    """creating a new list is semantically the most direct
    way to create an iterable to give to extend"""
    a_list.extend([element])


def perf2():
    '''
    Description :
    '''
    print(min(timeit.repeat(lambda: append_one([], 0))))
    print(min(timeit.repeat(lambda: extend_one([], 0))))


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

        # append appends a specified object at the end of the list:
        x = [1, 2, 3]
        x.append([4, 5])
        print(x)
        assert x == [1, 2, 3, [4, 5]]

        # extend extends the list by appending elements from the specified iterable:
        x = [1, 2, 3]
        x.extend([4, 5])
        print(x)
        assert x == [1, 2, 3, 4, 5]

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
        perf1()
        perf2()

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
