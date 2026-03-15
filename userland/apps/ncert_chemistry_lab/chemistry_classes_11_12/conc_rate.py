"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_11_12.conc_rate
"""

import math, re



class Chemistry_Classes_11_12:
    @staticmethod
    def conc_rate(c1, c2, t):
        rate = abs(c2 - c1) / t
        return {'Rate (M/s)': f'{rate:.4e}', 'Order': 'Assumed First Order'}
