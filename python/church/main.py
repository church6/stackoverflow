#!/usr/bin/python3
'''
# @filename     :  stackoverflow_3000_6288661.py
# @author       :  Copyright (C) Church.Zhong
# @date         :  Fri Nov 25 09:09:59 AM HKT 2022
# @title        :  Adding a user to a group in django
# @see          :  https://docs.python.org/3/library/datetime.html
# @require      :  Python 3.10.6 works well.
# @style        :  https://google.github.io/styleguide/pyguide.html
'''

# import os
import sys
import time

# import re
# import random
import argparse

from ids_stackoverflow import IDS_STACKOVERFLOW


def work():
    '''
    Description : work
    '''
    parser = argparse.ArgumentParser()
    parser.add_argument('--id', '-i', type=int, action='store', default=None, dest='id', help='id')
    results = parser.parse_args()

    if results.id is None:
        sys.exit(0)

    stackoverflow_id = results.id
    count = 1
    for key, value in IDS_STACKOVERFLOW.items():
        if stackoverflow_id in value:
            print(key)
            print(stackoverflow_id)
            page = int(key.split('_')[1])
            order = value.index(stackoverflow_id)
            count = count + (page - 1) * 50 + order
            answers = __import__(f'stackoverflow_{count:>04d}_{stackoverflow_id}')
            answers.verify()


def main():
    '''
    Description : main
    '''
    start = time.time()
    work()
    print(f'# elapsed time:{time.time() - start}')


if __name__ == "__main__":
    main()
