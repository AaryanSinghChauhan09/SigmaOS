"""
Auto-split from userland\apps\ncert_chemistry_lab.py — Chemistry_Classes_11_12.chromatog
"""

import math, re



class Chemistry_Classes_11_12:
    @staticmethod
    def chromatog(ds, dsp):
        rf = dsp / ds
        return {'Rf Value': _r(rf, 3), 'Status': 'Success' if rf < 1 else 'Error'}
