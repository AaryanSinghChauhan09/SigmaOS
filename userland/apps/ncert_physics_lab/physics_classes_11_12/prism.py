"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.prism
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def prism(a, d):
        n = math.sin(math.radians((a + d) / 2)) / math.sin(math.radians(a / 2))
        return {'Refr. Index n': _r(n, 3)}
