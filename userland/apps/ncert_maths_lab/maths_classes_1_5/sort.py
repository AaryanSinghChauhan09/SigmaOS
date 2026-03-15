"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_1_5.sort
"""

import math, random



class Maths_Classes_1_5:
    @staticmethod
    def sort(s):
        n = [int(x) for x in str(s).split(',')]
        return {'Sorted': sorted(n)}
