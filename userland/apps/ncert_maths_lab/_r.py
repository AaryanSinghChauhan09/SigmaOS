"""
Auto-split from userland\apps\ncert_maths_lab.py — _r
"""

import math, random



def _r(x, d=4):
    try:
        return float(('{:.' + str(int(d)) + 'f}').format(float(x)))
    except:
        return x
