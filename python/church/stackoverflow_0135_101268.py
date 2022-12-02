#!/usr/bin/python3
# pylint: disable=C0209
'''
# @filename     :  stackoverflow_0135_101268.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  Hidden features of Python [closed]
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

from colors import FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_YELLOW, ENDCOLOR, LINE


r'''
<ul>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#111176">Argument Unpacking</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#112303">Braces</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101945">Chaining Comparison Operators</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101447">Decorators</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#113198">Default Argument Gotchas / Dangers of Mutable Default arguments</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#102062">Descriptors</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#111970">Dictionary default <code>.get</code> value</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#102065">Docstring Tests</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python/112316#112316">Ellipsis Slicing Syntax</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#117116">Enumeration</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#114420">For/else</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#102202">Function as iter() argument</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101310">Generator expressions</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101276"><code>import this</code></a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#102037">In Place Value Swapping</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101840">List stepping</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#112286"><code>__missing__</code> items</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101537">Multi-line Regex</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#113164">Named string formatting</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101549">Nested list/generator comprehensions</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#108297">New types at runtime</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#113833"><code>.pth</code> files</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#1024693">ROT13 Encoding</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#143636">Regex Debugging</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#101739">Sending to Generators</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#168270">Tab Completion in Interactive Interpreter</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#116480">Ternary Expression</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#114157"><code>try/except/else</code></a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#3267903">Unpacking+<code>print()</code> function</a></li>
<li><a href="https://stackoverflow.com/questions/101268/hidden-features-of-python#109182"><code>with</code> statement</a></li>
</ul>
'''


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
