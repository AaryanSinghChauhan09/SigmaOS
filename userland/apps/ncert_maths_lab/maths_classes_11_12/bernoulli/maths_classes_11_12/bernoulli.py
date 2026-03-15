# Generated method: Maths_Classes_11_12.bernoulli
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def bernoulli(n, p, k):
        ni, ki = (int(n), int(k))
        pf = float(p)
        res = math.comb(ni, ki) * pf ** ki * (1.0 - pf) ** (ni - ki)
        return {'P(X=k)': _r(res, 6)}