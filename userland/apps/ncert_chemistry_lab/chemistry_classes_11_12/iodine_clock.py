"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_11_12.iodine_clock
"""

import math, re



class Chemistry_Classes_11_12:
    @staticmethod
    def iodine_clock(c, t):
        rate = c * (t / 10)
        return {'Time to Blue (s)': _r(50 / rate, 1), 'Observation': 'Clear -> Dark Blue'}
