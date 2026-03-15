"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_1_5.clock
"""

import math, random



class Maths_Classes_1_5:
    @staticmethod
    def clock(h, m):
        a = abs(30 * h - 5.5 * m)
        return {'Angle': min(a, 360 - a)}
