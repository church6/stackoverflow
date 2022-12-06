"""
sets      : 1.3439763600035803
regex     : 0.5356837599974824
translate : 0.5954884660022799
replace   : 1.5536595610028598

"""

import re
import string
import timeit

s = "string. With. Punctuation"
exclude = set(string.punctuation)
table = str.maketrans(dict.fromkeys(string.punctuation))
regex = re.compile('[%s]' % re.escape(string.punctuation))


def test_set(s):
    """function"""
    return ''.join(ch for ch in s if ch not in exclude)


def test_re(s):  # From Vinko's solution, with fix.
    """function"""
    return regex.sub('', s)


def test_trans(s):
    """function"""
    return s.translate(table)


def test_repl(s):  # From S.Lott's solution
    """function"""
    for c in string.punctuation:
        s = s.replace(c, "")
    return s


print("sets      :", timeit.Timer('f(s)', 'from __main__ import s,test_set as f').timeit(1000000))
print("regex     :", timeit.Timer('f(s)', 'from __main__ import s,test_re as f').timeit(1000000))
print("translate :", timeit.Timer('f(s)', 'from __main__ import s,test_trans as f').timeit(1000000))
print("replace   :", timeit.Timer('f(s)', 'from __main__ import s,test_repl as f').timeit(1000000))
