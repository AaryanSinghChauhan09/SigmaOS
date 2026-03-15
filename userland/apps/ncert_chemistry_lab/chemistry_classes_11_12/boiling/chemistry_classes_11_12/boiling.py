# Generated method: Chemistry_Classes_11_12.boiling
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def boiling(kb, m, i):
        dt = i * kb * m
        return {'Delta Tb': _r(dt, 3)}