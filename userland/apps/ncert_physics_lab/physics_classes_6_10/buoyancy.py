"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_6_10.buoyancy
"""

import math, random



class Physics_Classes_6_10:
    @staticmethod
    def buoyancy(v, d):
        return {'Upthrust (N)': _r(v * d * 9.81, 2)}
