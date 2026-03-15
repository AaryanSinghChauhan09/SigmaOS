# Generated method: Chemistry_Classes_11_12.gibbs
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def gibbs(h, s, t):
        g = h - t * (s / 1000)
        return {'Delta G (kJ)': _r(g, 2), 'Spontaneous': g < 0}