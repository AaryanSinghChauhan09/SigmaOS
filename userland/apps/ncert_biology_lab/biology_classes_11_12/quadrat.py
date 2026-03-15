"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_11_12.quadrat
"""

import math, random



class Biology_Classes_11_12:
    @staticmethod
    def quadrat(c, a):
        v = [int(x) for x in str(c).split(',')]
        return {'Density': _r(sum(v) / (len(v) * a), 2)}
