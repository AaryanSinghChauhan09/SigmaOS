"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_6_10.optics
"""

import math, random



class Physics_Classes_6_10:
    @staticmethod
    def optics(t, u, f):
        if 'mirror' in t.lower():
            v = 1 / (1 / f - 1 / u)
        else:
            v = 1 / (1 / f + 1 / u)
        return {'v (cm)': _r(v, 2), 'Mag': _r(-v / u if 'mirror' in t.lower() else v / u, 2)}
