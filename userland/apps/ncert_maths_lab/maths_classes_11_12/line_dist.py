"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.line_dist
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def line_dist(p1_s, p2_s):
        p1 = [float(x) for x in p1_s.split(',')]
        p2 = [float(x) for x in p2_s.split(',')]
        dist = math.sqrt(sum(((a - b) ** 2 for a, b in zip(p1, p2))))
        return {'Distance': _r(dist, 4)}
