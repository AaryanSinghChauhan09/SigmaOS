"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_11_12.tollen
"""

import math, re



class Chemistry_Classes_11_12:
    @staticmethod
    def tollen(s):
        if 'aldehyde' in s.lower() or 'glucose' in s.lower():
            return {'Observation': 'Silver Mirror formed', 'Result': 'Aldehyde PRESENT'}
        return {'Observation': 'No Mirror', 'Result': 'Ketone/Other'}
