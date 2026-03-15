"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.vector
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def vector(as_, bs):
        a = [float(x) for x in as_.split(',')]
        b = [float(x) for x in bs.split(',')]
        dot = sum((x * y for x, y in zip(a, b)))
        mag_b = math.sqrt(sum((x ** 2 for x in b)))
        return {'Proj_on_B': _r(dot / mag_b, 4)}
