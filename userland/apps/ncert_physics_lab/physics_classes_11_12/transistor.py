"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.transistor
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def transistor(vbe, ib, beta):
        ic = ib * 1e-06 * beta
        return {'I_c (mA)': _r(ic * 1000, 2), 'Status': 'Active' if vbe > 0.6 else 'Cut-off'}
