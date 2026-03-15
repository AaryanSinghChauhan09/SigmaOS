"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_6_10.salts
"""

import math, re



class Chemistry_Classes_6_10:
    @staticmethod
    def salts(c):
        return {'Solubility': 'High in Water' if 'NaCl' in c or 'K' in c else 'Lookup Table'}
