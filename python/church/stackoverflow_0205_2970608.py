#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0205_2970608.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  What are "named tuples" in Python?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

from math import sqrt
from collections import namedtuple

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


def example1():
    '''
    Description :
    '''
    pt1 = (1.0, 5.0)
    pt2 = (2.5, 1.5)

    # from math import sqrt
    line_length = sqrt((pt1[0] - pt2[0]) ** 2 + (pt1[1] - pt2[1]) ** 2)
    print(f'line_length = {line_length}')


def example2():
    '''
    Description :
    '''
    # from collections import namedtuple
    Point = namedtuple('Point', 'x y')
    pt1 = Point(1.0, 5.0)
    pt2 = Point(2.5, 1.5)

    line_length = sqrt((pt1.x - pt2.x) ** 2 + (pt1.y - pt2.y) ** 2)
    print(f'line_length = {line_length}')


def example3():
    '''
    Description :
    '''
    Point = namedtuple('Point', 'x y')
    pt1 = Point(1.0, 5.0)
    pt2 = Point(2.5, 1.5)

    # use index referencing
    line_length = sqrt((pt1[0] - pt2[0]) ** 2 + (pt1[1] - pt2[1]) ** 2)
    # use tuple unpacking
    x1, y1 = pt1
    print(f'line_length = {line_length}')


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
        example1()
        example2()
        example3()

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
