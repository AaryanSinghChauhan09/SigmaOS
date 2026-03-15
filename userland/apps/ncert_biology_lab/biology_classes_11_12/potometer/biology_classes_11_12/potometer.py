# Generated method: Biology_Classes_11_12.potometer
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def potometer(d, t):
        rate = d / t
        return {'Transpiration Rate': _r(rate, 2), 'Unit': 'cm/min'}