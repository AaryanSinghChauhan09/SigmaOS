"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_6_10.gravity
"""

import math, random



class Physics_Classes_6_10:
    @staticmethod
    def gravity(m, b):
        g = {'earth': 9.81, 'moon': 1.62, 'mars': 3.71, 'jupiter': 24.79}.get(b.lower(), 9.81)
        return {'Weight (N)': _r(m * g, 2)}
