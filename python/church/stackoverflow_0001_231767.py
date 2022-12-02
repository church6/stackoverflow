#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0001_231767.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  What does the "yield" keyword do in Python?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


def create_generator():
    '''
    Description : create_generator
    '''
    mylist = range(3)
    for i in mylist:
        yield i * i


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
        # Iterables
        mylist = [1, 2, 3]
        for i in mylist:
            print(i)
        for i in mylist:
            print(i)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')
        # Iterables
        mylist = [x * x for x in range(3)]
        for i in mylist:
            print(i)
        for i in mylist:
            print(i)

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer1::code3')
        # Generators
        mygenerator = (x * x for x in range(3))
        for i in mygenerator:
            print(i)
        for i in mygenerator:
            print(i)

    @staticmethod
    def code4():
        '''
        Description : code4
        '''
        print('Answer1::code4')
        mygenerator = create_generator()  # create a generator
        print(mygenerator)  # mygenerator is an object!
        # <generator object create_generator at 0xb7555c34>
        for i in mygenerator:
            print(i)
        for i in mygenerator:
            print(i)

    @staticmethod
    def verify():
        '''
        Description : verify
        '''
        print(f'{FOREGROUND_RED}{LINE}{ENDCOLOR}')
        Answer1.code1()
        Answer1.code2()
        Answer1.code3()
        Answer1.code4()
        print(f'{FOREGROUND_RED}{LINE}{ENDCOLOR}')


def f123():
    '''
    Description : f123
    '''
    yield 1 * 1 * 1
    yield 2 * 2 * 2
    yield 3 * 3 * 3


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
        for item in f123():
            print(item)

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


def some_function():
    '''
    Description : some_function
    '''
    for i in range(4):
        yield i


class MyIterator:
    '''
    Description : iterator
    '''

    def __init__(self):
        '''
        Description : __init__
        '''
        # Start at -1 so that we get 0 when we add 1 below.
        self.count = -1

    # The __iter__ method will be called once by the 'for' loop.
    # The rest of the magic happens on the object returned by this method.
    # In this case it is the object itself.
    def __iter__(self):
        '''
        Description : __iter__
        '''
        return self

    # The next method will be called repeatedly by the 'for' loop
    # until it raises StopIteration.
    # def next(self):
    def __next__(self):
        '''
        Description : next
        '''
        self.count += 1
        if self.count < 4:
            return self.count
        else:
            # A StopIteration exception is raised
            # to signal that the iterator is done.
            # This is caught implicitly by the 'for' loop.
            raise StopIteration


def some_func():
    '''
    Description : some_func
    '''
    return MyIterator()


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
        for i in some_function():
            print(i)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer3::code2')
        for i in some_func():
            print(i)

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer3::code3')
        iterator = some_func()
        try:
            while 1:
                print(iterator.__next__())
        except StopIteration:
            pass

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


def make_range(size):
    '''
    Description : makeRange
    '''
    # return 0,1,2,...,n-1
    i = 0
    while i < size:
        yield i
        i += 1


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
        for i in make_range(5):
            print(i)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer4::code2')
        print([x + 10 for x in make_range(5)])

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
