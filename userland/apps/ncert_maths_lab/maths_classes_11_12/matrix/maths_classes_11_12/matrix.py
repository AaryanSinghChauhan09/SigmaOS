# Generated method: Maths_Classes_11_12.matrix
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def matrix(m):
        r = [[float(x) for x in row.split(',')] for row in m.split(';')]
        return {'Det': r[0][0] * r[1][1] - r[0][1] * r[1][0]}