# Generated method: Physics_Classes_11_12.screw_gauge
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def screw_gauge(psr, hsd, lc):
        total = psr + hsd * lc
        return {'Diameter (mm)': _r(total, 3)}