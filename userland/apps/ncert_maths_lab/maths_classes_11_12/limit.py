"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.limit
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def limit(n, a):
        nf, af = (float(n), float(a))
        return {'Val': nf * af ** (nf - 1)}
