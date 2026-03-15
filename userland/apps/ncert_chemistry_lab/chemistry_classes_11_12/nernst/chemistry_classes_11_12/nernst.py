# Generated method: Chemistry_Classes_11_12.nernst
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def nernst(e0, n, q):
        e = e0 - 0.0591 / n * math.log10(q)
        return {'E_Cell (V)': _r(e, 4)}