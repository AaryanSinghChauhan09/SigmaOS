# Generated method: Chemistry_Classes_11_12.molarity
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def molarity(m, mw, v):
        mol = m / mw
        return {'Molarity (M)': _r(mol / v, 3)}