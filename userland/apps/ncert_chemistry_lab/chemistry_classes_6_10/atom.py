"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_6_10.atom
"""

import math, re



class Chemistry_Classes_6_10:
    @staticmethod
    def atom(z):
        d = {1: 'H', 2: 'He', 6: 'C', 7: 'N', 8: 'O', 11: 'Na', 17: 'Cl', 26: 'Fe', 79: 'Au'}
        return {'Symbol': d.get(z, '?'), 'Z': z}
