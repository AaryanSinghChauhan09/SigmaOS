"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_11_12.ean
"""

import math, re



class Chemistry_Classes_11_12:
    @staticmethod
    def ean(z, ox, cn):
        res = z - ox + 2 * cn
        return {'EAN': int(res)}
