#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0279_1207406.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  How to remove items from a list while iterating?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

from itertools import filterfalse

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


def determine(i):
    '''
    Description :
    '''
    return i % 2 == 0


def example():
    '''
    Description :
    '''
    somelist = [0, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
    somelist = [x for x in somelist if not determine(x)]
    print(somelist)
    # Or, by assigning to the slice somelist[:], you can mutate the existing list to contain only the items you want:
    somelist = [0, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
    somelist[:] = [x for x in somelist if not determine(x)]
    print(somelist)
    # This approach could be useful if there are other references to somelist that need to reflect the changes.

    # Instead of a comprehension, you could also use itertools.
    # In Python 2:
    # from itertools import ifilterfalse
    # somelist[:] = ifilterfalse(determine, somelist)
    # Or in Python 3:
    # from itertools import filterfalse
    somelist = [0, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
    somelist[:] = filterfalse(determine, somelist)
    print(somelist)


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
        example()

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
