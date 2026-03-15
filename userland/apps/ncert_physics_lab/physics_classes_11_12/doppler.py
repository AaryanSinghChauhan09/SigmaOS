"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.doppler
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def doppler(f, vs, vo):
        v = 343
        return {'Observed F (Hz)': _r(f * (v + vo) / (v - vs), 2)}
