"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_11_12.pollen_grow
"""

import math, random



class Biology_Classes_11_12:
    @staticmethod
    def pollen_grow(s, h):
        if 8 < s < 12 and h >= 1:
            return {'Status': 'TUBE FORMATION OBSERVED', 'Length': 'Significant'}
        return {'Status': 'No Growth', 'Reason': 'Sugar/Time Mismatch'}
