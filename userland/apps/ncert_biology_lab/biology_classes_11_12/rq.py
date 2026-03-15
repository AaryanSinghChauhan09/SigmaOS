"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_11_12.rq
"""

import math, random



class Biology_Classes_11_12:
    @staticmethod
    def rq(c, o):
        r = c / o
        return {'RQ': _r(r, 2), 'Sub': 'Carb' if 0.95 < r < 1.05 else 'Fat/Protein'}
