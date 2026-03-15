"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.photo
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def photo(lam, phi):
        e = 1240 / lam
        return {'E_ph (eV)': _r(e, 2), 'Emission': e > phi}
