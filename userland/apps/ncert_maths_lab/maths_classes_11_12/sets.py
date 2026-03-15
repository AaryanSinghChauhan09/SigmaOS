"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.sets
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def sets(as_, bs, us):
        a = set(as_.split(','))
        b = set(bs.split(','))
        u = set(us.split(','))
        lhs = u.difference(a.union(b))
        rhs = u.difference(a).intersection(u.difference(b))
        return {'Verified': lhs == rhs}
