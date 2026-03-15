"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.ydse
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def ydse(lam, d, big_d):
        w = lam * 1e-09 * big_d / (d * 0.001)
        return {'Width (mm)': _r(w * 1000, 3)}
