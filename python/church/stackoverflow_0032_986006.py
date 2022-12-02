#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0032_986006.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  How do I pass a variable by reference?
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


# Arguments are passed by assignment. The rationale behind this is twofold:

# the parameter passed in is actually a reference to an object (but the reference is passed by value)
# some data types are mutable, but others aren't

# So:
# If you pass a mutable object into a method, the method gets a reference to that same object and you can mutate it to your heart's delight,
# but if you rebind the reference in the method, the outer scope will know nothing about it, and after you're done, the outer reference will still point at the original object.

# If you pass an immutable object to a method, you still can't rebind the outer reference, and you can't even mutate the object.

# List - a mutable type
# Let's try to modify the list that was passed to a method:
def try_to_change_list_contents(the_list):
    '''
    Description :
    '''
    print('got', the_list)
    the_list.append('four')
    print('changed to', the_list)


# String - an immutable type
# It's immutable, so there's nothing we can do to change the contents of the string
# Now, let's try to change the reference
def try_to_change_string_reference(the_string):
    '''
    Description :
    '''
    print('got', the_string)
    the_string = 'In a kingdom by the sea'
    print('set to', the_string)


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
        outer_list = ['one', 'two', 'three']
        print('before, outer_list =', outer_list)
        try_to_change_list_contents(outer_list)
        print('after, outer_list =', outer_list)

    @staticmethod
    def code2():
        '''
        Description : code2
        '''
        print('Answer1::code2')
        outer_string = 'It was many and many a year ago'
        print('before, outer_string =', outer_string)
        try_to_change_string_reference(outer_string)
        print('after, outer_string =', outer_string)

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
