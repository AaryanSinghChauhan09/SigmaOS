# Generated method: Maths_Classes_11_12.p_and_c
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def p_and_c(n, r):
        n, r = (int(n), int(r))
        return {'nPr': math.perm(n, r), 'nCr': math.comb(n, r)}