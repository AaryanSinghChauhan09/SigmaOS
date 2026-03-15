# Generated method: Chemistry_Classes_11_12.osmosis
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def osmosis(m, tc, i):
        pi = i * m * 0.0821 * (tc + 273.15)
        return {'Pi (atm)': _r(pi, 2)}