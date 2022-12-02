#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0006_38987.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  How do I merge two dictionaries in a single expression?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

# Performance Analysis
from timeit import repeat
from itertools import chain

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


# In Python 2, (or 3.4 or lower) write a function:


def merge_two_dicts(x, y):
    '''
    Description : merge_two_dicts
    '''
    z = x.copy()  # start with keys and values of x
    z.update(y)  # modifies z with keys and values of y
    return z


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
        x = {'a': 1, 'b': 2}
        y = {'b': 3, 'c': 4}

        # In Python 3.9.0 or greater (released 17 October 2020, PEP-584, discussed here):
        z = x | y
        print(z)
        # In Python 3.5 or greater:
        z = {**x, **y}
        print(z)
        # In Python 2, (or 3.4 or lower) write a function:
        z = merge_two_dicts(x, y)
        print(z)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')

        # Performance Analysis
        x = dict.fromkeys('abcdefg')
        y = dict.fromkeys('efghijk')
        t = min(repeat(lambda: x | y))
        print(t)
        t = min(repeat(lambda: {**x, **y}))
        print(t)
        t = min(repeat(lambda: merge_two_dicts(x, y)))
        print(t)
        t = min(repeat(lambda: {k: v for d in (x, y) for k, v in d.items()}))
        print(t)
        t = min(repeat(lambda: dict(chain(x.items(), y.items()))))
        print(t)
        t = min(repeat(lambda: dict(item for d in (x, y) for item in d.items())))
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
