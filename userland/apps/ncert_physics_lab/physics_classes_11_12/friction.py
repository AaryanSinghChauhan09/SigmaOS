"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.friction
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def friction(m, a):
        mu = math.tan(math.radians(a))
        force = m * 9.81 * math.sin(math.radians(a))
        return {'Coeff mu': _r(mu, 3), 'Status': 'Impending Motion' if a > 15 else 'Static'}
