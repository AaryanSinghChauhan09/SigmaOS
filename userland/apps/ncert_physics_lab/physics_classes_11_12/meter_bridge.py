"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.meter_bridge
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def meter_bridge(r, l):
        x = r * (100 - l) / l
        return {'Unknown X (Ω)': _r(x, 2)}
