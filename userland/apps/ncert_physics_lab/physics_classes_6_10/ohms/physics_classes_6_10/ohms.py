# Generated method: Physics_Classes_6_10.ohms
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def ohms(v, r):
        return {'I (A)': _r(v / r), 'P (W)': _r(v ** 2 / r)}