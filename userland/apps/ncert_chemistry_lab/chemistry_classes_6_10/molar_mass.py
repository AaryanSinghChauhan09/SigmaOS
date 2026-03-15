"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_6_10.molar_mass
"""

import math, re



class Chemistry_Classes_6_10:
    @staticmethod
    def molar_mass(f):
        mats = re.findall('([A-Z][a-z]?)(\\d*)', f)
        total = 0
        elem = {'H': 1.0, 'C': 12.0, 'O': 16.0, 'Na': 23.0, 'S': 32.1, 'Cl': 35.5}
        for s, c in mats:
            n = int(c) if c else 1
            total += elem.get(s, 0) * n
        return {'Molar Mass': _r(total, 2)}
