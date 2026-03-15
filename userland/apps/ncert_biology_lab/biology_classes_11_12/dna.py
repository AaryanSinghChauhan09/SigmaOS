"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_11_12.dna
"""

import math, random



class Biology_Classes_11_12:
    @staticmethod
    def dna(s):
        d = {'A': 'T', 'T': 'A', 'C': 'G', 'G': 'C'}
        res = [str(d.get(b.upper(), b)) for b in s]
        return {'Complement': ''.join(res)}
