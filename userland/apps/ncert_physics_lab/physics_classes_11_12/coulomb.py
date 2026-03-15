"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.coulomb
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def coulomb(q1, q2, r):
        k = 9000000000.0
        q1 *= 1e-06
        q2 *= 1e-06
        r /= 100
        return {'Force (N)': _r(k * (q1 * q2) / r ** 2, 3)}
