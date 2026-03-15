"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_11_12.pollen
"""

import math, random



class Biology_Classes_11_12:
    @staticmethod
    def pollen(s, t):
        if 5 < s < 15 and t > 20:
            return {'Status': 'SUCCESSFUL GERMINATION', 'Tube Length': 'Long'}
        return {'Status': 'FAILED', 'Reason': 'Sugar Mismatch'}
