"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_1_5.add
"""

import math, random



class Maths_Classes_1_5:
    @staticmethod
    def add(a, b):
        return {'Sum': a + b, 'Carry': a % 10 + b % 10 >= 10}
