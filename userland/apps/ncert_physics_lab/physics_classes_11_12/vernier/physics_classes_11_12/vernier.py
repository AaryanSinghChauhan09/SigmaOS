# Generated method: Physics_Classes_11_12.vernier
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def vernier(msr, vsd, lc):
        total = msr + vsd * lc
        return {'Thickness (cm)': _r(total, 3)}