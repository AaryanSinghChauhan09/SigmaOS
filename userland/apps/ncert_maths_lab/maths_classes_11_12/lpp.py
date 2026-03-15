"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.lpp
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def lpp(obj, lim):
        res = max(obj[0] * lim, obj[1] * lim)
        return {'Max Z': res, 'Optimal Point': f'({lim} or 0)'}
