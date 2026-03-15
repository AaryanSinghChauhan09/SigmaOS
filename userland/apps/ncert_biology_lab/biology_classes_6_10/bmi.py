"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_6_10.bmi
"""

import math, random



class Biology_Classes_6_10:
    @staticmethod
    def bmi(w, h):
        b = w / h ** 2
        return {'BMI': _r(b, 1), 'Cat': 'Normal' if 18.5 <= b < 25 else 'Other'}
