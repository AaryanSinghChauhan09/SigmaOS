# Generated method: Maths_Classes_11_12.progression
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def progression(t, a, dr, n):
        n = int(n)
        if 'AP' in t.upper():
            tn = a + (n - 1) * dr
            sn = n / 2 * (2 * a + (n - 1) * dr)
            return {'n-th term': tn, 'Sum of n': sn}
        else:
            tn = a * dr ** (n - 1)
            sn = a * (dr ** n - 1) / (dr - 1) if dr != 1 else a * n
            return {'n-th term': _r(tn, 2), 'Sum of n': _r(sn, 2)}