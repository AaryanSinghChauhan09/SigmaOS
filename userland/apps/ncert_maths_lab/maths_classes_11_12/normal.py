"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.normal
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def normal(x, mu, s):
        z = (x - mu) / s
        return {'Z-score': _r(z, 2), 'Status': 'Outlier' if abs(z) > 3 else 'Within Normal Range'}
