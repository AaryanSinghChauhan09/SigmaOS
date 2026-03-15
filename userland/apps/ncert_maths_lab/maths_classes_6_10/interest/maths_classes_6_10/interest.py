# Generated method: Maths_Classes_6_10.interest
import math, random

class Maths_Classes_6_10:
    @staticmethod
    def interest(p, r, t):
        return {'SI': p * r * t / 100, 'CI': _r(p * (1 + r / 100) ** t - p, 2)}