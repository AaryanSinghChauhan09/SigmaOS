# Generated method: Chemistry_Classes_11_12.arrhenius
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def arrhenius(a, ea, t):
        k = a * math.exp(-ea / (0.00831 * t))
        return {'k': f'{k:.4e}'}