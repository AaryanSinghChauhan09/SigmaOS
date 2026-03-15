"""
Auto-split from userland\apps\ncert_chemistry_lab.py — _r
"""

import math, re



def _r(x, d=4):
    try:
        return float(('{:.' + str(int(d)) + 'f}').format(float(x)))
    except:
        return x
