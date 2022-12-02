#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0019_3437059.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  Does Python have a string 'contains' substring method?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

import timeit
from dis import dis

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
        # Use the in operator:
        needle = 'blah'
        if needle not in ['a', 'b', 'c']:
            print(f'no such {needle}')
        needle = 'b'
        if needle in ['a', 'b', 'c']:
            print(f'found such {needle}')

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


def in_(s, other):
    '''
    Description :
    '''
    return other in s


def contains(s, other):
    '''
    Description :
    '''
    return s.__contains__(other)


def find(s, other):
    '''
    Description :
    '''
    return s.find(other) != -1


def index(s, other):
    '''
    Description :
    '''
    try:
        s.index(other)
    except ValueError:
        return False
    else:
        return True


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
        perf_dict = {
            'in:True': min(timeit.repeat(lambda: in_('superstring', 'str'))),
            'in:False': min(timeit.repeat(lambda: in_('superstring', 'not'))),
            '__contains__:True': min(timeit.repeat(lambda: contains('superstring', 'str'))),
            '__contains__:False': min(timeit.repeat(lambda: contains('superstring', 'not'))),
            'find:True': min(timeit.repeat(lambda: find('superstring', 'str'))),
            'find:False': min(timeit.repeat(lambda: find('superstring', 'not'))),
            'index:True': min(timeit.repeat(lambda: index('superstring', 'str'))),
            'index:False': min(timeit.repeat(lambda: index('superstring', 'not'))),
        }
        for key, value in perf_dict.items():
            print(f'{key}:{value}')

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer3::code2')
        dis(lambda: 'a' in 'b')
        dis(lambda: 'b'.__contains__('a'))

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
