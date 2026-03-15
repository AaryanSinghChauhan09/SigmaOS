# Generated method: Maths_Classes_6_10.frustum
import math, random

class Maths_Classes_6_10:
    @staticmethod
    def frustum(r1, r2, h):
        v = 1 / 3 * math.pi * h * (r1 ** 2 + r2 ** 2 + r1 * r2)
        return {'Volume': _r(v, 2)}