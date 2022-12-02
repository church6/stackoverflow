#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0073_13411544.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  Delete a column from a Pandas DataFrame
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

import pandas as pd

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

        dframe = pd.DataFrame.from_dict(
            data={'A': [1, 2, 3], 'B': [4, 5, 6], 'C': [7, 8, 9]}, orient='index', columns=['one', 'two', 'three']
        )
        # dframe = DataFrame.from_items([('A', [1, 2, 3]), ('B', [4, 5, 6]), ('C', [7,8, 9])], orient='index', columns=['one', 'two', 'three'])
        print(dframe)

        column_name = 'one'
        # The best way to do this in Pandas is to use drop:
        dframe = dframe.drop(column_name, axis=1)
        print(dframe)

        dframe = pd.DataFrame.from_dict(
            data={'A': [1, 2, 3], 'B': [4, 5, 6], 'C': [7, 8, 9]}, orient='index', columns=['one', 'two', 'three']
        )
        column_name = 'one'
        # where 1 is the axis number (0 for rows and 1 for columns.)
        # To delete the column without having to reassign dframe you can do:
        dframe.drop(column_name, axis=1, inplace=True)
        print(dframe)
        return

        dframe = pd.DataFrame.from_dict(
            data={'A': [1, 2, 3], 'B': [4, 5, 6], 'C': [7, 8, 9]}, orient='index', columns=['one', 'two', 'three']
        )
        # Finally, to drop by column number instead of by column label, try this to delete, e.g. the 1st, 2nd and 4th columns:
        dframe = dframe.drop(dframe.columns[[0, 1, 3]], axis=1)  # dframe.columns is zero-based pd.Index
        print(dframe)

        dframe = pd.DataFrame.from_dict(
            data={'A': [1, 2, 3], 'B': [4, 5, 6], 'C': [7, 8, 9]}, orient='index', columns=['one', 'two', 'three']
        )
        # Also working with "text" syntax for the columns:
        dframe.drop(['column_nameA', 'column_nameB'], axis=1, inplace=True)
        # Note: Introduced in v0.21.0 (October 27, 2017), the drop() method accepts index/columns keywords as an alternative to specifying the axis.
        print(dframe)

        # So we can now just do:
        dframe = dframe.drop(columns=['column_nameA', 'column_nameB'])

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
