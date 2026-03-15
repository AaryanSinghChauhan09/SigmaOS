# Generated method: Maths_Classes_11_12.mean_dev
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def mean_dev(s):
        v = [float(x) for x in str(s).split(',')]
        mean = sum(v) / len(v)
        md = sum((abs(x - mean) for x in v)) / len(v)
        return {'Mean': mean, 'Mean Deviation': _r(md, 2)}