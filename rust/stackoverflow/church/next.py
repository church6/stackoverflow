#!/usr/bin/python3
"""
# @filename           :  next.py
# @author             :  Copyright (C) Church.Zhong
# @date               :  Thu Nov 24 12:51:13 PM HKT 2022
# @function           :
# @see                :  https://docs.python.org/3/library/datetime.html
# @require            :  Python 3.10.6 works well.
# @style              :  https://google.github.io/styleguide/pyguide.html
"""

import os
import sys

# import time
import re

# from datetime import datetime
# from datetime import timedelta
# import argparse

from ids_stackoverflow import IDS_STACKOVERFLOW

FOREGROUND_BLACK = '\033[30m'
FOREGROUND_RED = '\033[31m'
FOREGROUND_GREEN = '\033[32m'
FOREGROUND_YELLOW = '\033[33m'
FOREGROUND_BLUE = '\033[34m'
FOREGROUND_MAGENTA = '\033[35m'
FOREGROUND_CYAN = '\033[36m'
FOREGROUND_LIGHT_GRAY = '\033[37m'
FOREGROUND_GRAY = '\033[90m'
FOREGROUND_LIGHT_RED = '\033[91m'
FOREGROUND_LIGHT_GREEN = '\033[92m'
FOREGROUND_LIGHT_YELLOW = '\033[93m'
FOREGROUND_LIGHT_BLUE = '\033[94m'
FOREGROUND_LIGHT_MAGENTA = '\033[95m'
FOREGROUND_LIGHT_CYAN = '\033[96m'
ENDCOLOR = '\033[0m'


ID_REGSTR = r'^mod stackoverflow_([0-9]+);$'
ID_REGEX = re.compile(ID_REGSTR)


def work():
    '''
    Description : work
    '''
    dirname = os.path.dirname(__file__)
    # print(f'{FOREGROUND_GREEN}# dirname = {dirname}{ENDCOLOR}')
    temporary = dirname + '/tmp_stackoverflow.txt'

    next_page = ''  # Invalid
    next_id = -1  # Invalid
    next_count = -1  # Invalid
    if not os.path.exists(temporary):
        # print(f'{temporary}: No such file or directory')
        next_page = '001'
        next_id = IDS_STACKOVERFLOW['html_' + next_page][0]
        next_count = 1
    else:
        current_page = ''  # Invalid
        current_id = -1  # Invalid
        current_count = -1  # Invalid
        with open(temporary, mode='r', encoding='utf-8') as rfh:
            current = rfh.read().rstrip()
            [current_page, current_id, current_count] = current.split("\t")
            current_id = int(current_id)
            current_count = int(current_count)

        if current_page not in ['', '060']:
            current_index = IDS_STACKOVERFLOW['html_' + current_page].index(current_id)
            next_index = 0
            if 0 <= current_index < 49:
                next_page = current_page
                next_index = current_index + 1
            elif current_index == 49:
                next_page = f'{(int(current_page) + 1):>03d}'
                next_index = 0
            else:
                sys.exit(0)

            key = 'html_' + next_page
            next_id = IDS_STACKOVERFLOW[key][next_index]
            next_count = current_count + 1

    if -1 != next_id:
        # print(f'{FOREGROUND_RED}{next_count:>04d}_{next_id}{ENDCOLOR}')
        print(f"{next_page}\t{next_id}\t{next_count:>04d}", end='')
    else:
        pass


def main():
    """
    Description : main
    """
    # start = time.time()
    work()

    # print(f'# elapsed time:{time.time() - start}')


if __name__ == "__main__":
    main()

# t=next.py;flake8 --max-line-length 128 --indent-size 4 ${t};pylint --max-line-length=128 --indent-after-paren=4 ${t};
