"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.differential
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def differential(k, y0, x):
        res = float(y0) * math.exp(float(k) * float(x))
        return {'Solution y(x)': _r(res, 2), 'Type': 'Exponential Growth/Decay'}
