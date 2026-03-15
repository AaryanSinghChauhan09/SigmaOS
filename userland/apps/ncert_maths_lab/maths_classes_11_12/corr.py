"""
Auto-split from userland\apps\ncert_maths_lab.py — Maths_Classes_11_12.corr
"""

import math, random



class Maths_Classes_11_12:
    @staticmethod
    def corr(xs, ys):
        x = [float(i) for i in xs.split(',')]
        y = [float(i) for i in ys.split(',')]
        mx = sum(x) / len(x)
        my = sum(y) / len(y)
        num = sum(((i - mx) * (j - my) for i, j in zip(x, y)))
        den = math.sqrt(sum(((i - mx) ** 2 for i in x)) * sum(((j - my) ** 2 for j in y)))
        return {'Correlation r': _r(num / den, 4) if den else 0}
