#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0004_100003.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  What are metaclasses in Python?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


class ObjectCreator(object):
    '''
    Description : ObjectCreator
    '''

    pass


def choose_class(name):
    '''
    Description : choose_class
    '''
    if name == 'foo':

        class Foo(object):
            '''
            Description : Foo
            '''

            pass

        return Foo  # return the class, not an instance
    else:

        class Bar(object):
            '''
            Description : Bar
            '''

            pass

        return Bar


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
        # Classes as objects
        my_object = ObjectCreator()
        print(my_object)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')
        # Creating classes dynamically
        MyClass = choose_class('foo')
        print(MyClass)  # the function returns a class, not an instance
        print(MyClass())  # you can create an object from this class

    @staticmethod
    def code3():
        '''
        Description : code3
        '''
        print('Answer1::code3')
        print(type(1))
        print(type("1"))
        print(type(ObjectCreator))
        print(type(ObjectCreator()))

    @staticmethod
    def code4():
        '''
        Description : code4
        '''
        print('Answer1::code4')
        # the function type is in fact a metaclass.
        # type is the metaclass Python uses to create all classes behind the scenes.
        age = 35
        print(age.__class__)
        name = 'bob'
        print(name.__class__)

        def foo():
            pass

        print(foo.__class__)

        class Bar(object):
            pass

        b = Bar()
        print(b.__class__)

        # Now, what is the __class__ of any __class__ ?
        print(age.__class__.__class__)
        print(name.__class__.__class__)
        print(foo.__class__.__class__)
        print(b.__class__.__class__)

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
