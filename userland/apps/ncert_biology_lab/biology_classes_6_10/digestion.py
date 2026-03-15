"""
Auto-split from userland\apps\ncert_biology_lab.py — Biology_Classes_6_10.digestion
"""

import math, random



class Biology_Classes_6_10:
    @staticmethod
    def digestion(ph, t):
        if 1.5 < ph < 2.5 and 35 < t < 40:
            return {'Status': 'ACTIVE (Gastric)'}
        return {'Status': 'INACTIVE'}
