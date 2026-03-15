"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.visco
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def visco(r, rs, rf, v):
        r /= 1000
        eta = 2 * r ** 2 * (rs - rf) * 9.81 / (9 * v)
        return {'Viscosity (Pa.s)': _r(eta, 4)}
