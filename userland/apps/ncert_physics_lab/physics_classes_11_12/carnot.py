"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.carnot
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def carnot(th, tc):
        eff = 1 - tc / th
        return {'Efficiency %': _r(eff * 100, 2)}
