# Generated method: Maths_Classes_6_10.quad
import math, random

class Maths_Classes_6_10:
    @staticmethod
    def quad(a, b, c):
        d = b ** 2 - 4 * a * c
        if d < 0:
            return {'Roots': 'Complex'}
        return {'x1': _r((-b + math.sqrt(d)) / (2 * a)), 'x2': _r((-b - math.sqrt(d)) / (2 * a))}