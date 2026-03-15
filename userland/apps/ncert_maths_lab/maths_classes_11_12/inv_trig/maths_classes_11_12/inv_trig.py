# Generated method: Maths_Classes_11_12.inv_trig
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def inv_trig(v, f):
        v = float(v)
        if 'sin' in f.lower():
            res = math.degrees(math.asin(v))
        elif 'cos' in f.lower():
            res = math.degrees(math.acos(v))
        else:
            res = 0
        return {'Principal Val (deg)': _r(res, 2)}