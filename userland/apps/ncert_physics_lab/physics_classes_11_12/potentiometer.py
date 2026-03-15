"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.potentiometer
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def potentiometer(e1, l1, l2):
        e2 = e1 * (l2 / l1)
        return {'EMF E2 (V)': _r(e2, 3)}
