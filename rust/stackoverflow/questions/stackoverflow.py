#!/usr/bin/python3
"""
# @filename           :  stackoverflow.py
# @author             :  Copyright (C) Church.Zhong
# @date               :  Fri Nov 11 09:14:14 AM HKT 2022
# @function           :
# @see                :  https://docs.python.org/3/library/datetime.html
# @require            :  Python 3.10.6 works well.
# @style              :  https://google.github.io/styleguide/pyguide.html
"""

# import os
# import sys
import time

# import re
import random

# from datetime import datetime
# from datetime import timedelta
# import argparse


import requests


def pages(url, i):
    """
    Description : pages
    """
    filename = f'{i:0>3}.html'
    print(filename)

    headers = {
        'Connection': 'keep-alive',
        'upgrade-insecure-requests': '1',
        'accept-encoding': 'gzip, deflate, br',
        'accept-language': 'en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7',
        'cache-control': 'max-age=0',
    }
    response = False
    response = requests.request("GET", url, headers=headers, timeout=200)
    print(response.status_code)
    with open(filename, mode='w', encoding='utf-8') as wfh:
        wfh.write(response.text)


def work():
    """
    Description : work
    """
    print('hello world')

    urls = ['https://stackoverflow.com/questions/tagged/rust?tab=Votes']
    for i in list(range(2, 61)):
        # print(i)
        urls.append(f'https://stackoverflow.com/questions/tagged/rust?tab=votes&page={i}&pagesize=50')

    for index, url in enumerate(urls):
        print(f'{index},{url}')
        # pass
        pages(url, index + 1)
        time.sleep(random.randint(10, 20) / 100)


def main():
    """
    Description : main
    """
    start = time.time()
    work()
    print(f'# elapsed time:{time.time() - start}')


if __name__ == "__main__":
    main()

# t=stackoverflow.py;flake8 --max-line-length 128 --indent-size 4 ${t};pylint --max-line-length=128 --indent-after-paren=4 ${t};
