# Generated method: Maths_Classes_11_12.integ
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def integ(n, l, u):
        res = u ** (n + 1) / (n + 1) - l ** (n + 1) / (n + 1)
        return {'Result': _r(res, 4)}