# Generated method: Maths_Classes_11_12.binomial
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def binomial(n, p, r):
        n, p, r = (int(n), float(p), int(r))
        comb = math.comb(n, r)
        prob = comb * p ** r * (1.0 - p) ** (n - r)
        return {'P(X=r)': _r(prob, 6)}