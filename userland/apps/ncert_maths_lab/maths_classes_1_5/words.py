"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_1_5.words
"""

import math, random



class Maths_Classes_1_5:
    @staticmethod
    def words(n):
        d = {1: 'One', 2: 'Two', 3: 'Three', 5: 'Five', 7: 'Seven'}
        return {'Word': d.get(int(n), '?')}
