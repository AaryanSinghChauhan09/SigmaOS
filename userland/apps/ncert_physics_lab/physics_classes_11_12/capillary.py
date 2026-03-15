"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.capillary
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def capillary(r_mm, h_cm):
        r = r_mm / 1000
        h = h_cm / 100
        rho = 1000
        g = 9.81
        t = rho * g * r * h / 2
        return {'Surface Tension (N/m)': _r(t, 4)}
