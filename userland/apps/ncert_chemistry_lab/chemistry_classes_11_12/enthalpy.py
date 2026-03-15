"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_11_12.enthalpy
"""

import math, re



class Chemistry_Classes_11_12:
    @staticmethod
    def enthalpy(n, dt):
        dh = -(100 * 4.184 * dt) / n
        return {'delta_H (kJ/mol)': _r(dh / 1000, 2), 'Note': 'Exothermic Neutralization'}
