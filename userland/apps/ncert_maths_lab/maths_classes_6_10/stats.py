"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_6_10.stats
"""

import math, random



class Maths_Classes_6_10:
    @staticmethod
    def stats(s):
        v = [float(x) for x in str(s).split(',')]
        m = sum(v) / len(v)
        return {'Mean': _r(m, 2), 'SD': _r(math.sqrt(sum(((x - m) ** 2 for x in v)) / len(v)), 2)}
