# Generated method: Maths_Classes_11_12.cross
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def cross(as_, bs):
        a = [float(x) for x in as_.split(',')]
        b = [float(x) for x in bs.split(',')]
        if len(a) != 3 or len(b) != 3:
            return {'Error': '3D Vectors Required'}
        i = a[1] * b[2] - a[2] * b[1]
        j = -(a[0] * b[2] - a[2] * b[0])
        k = a[0] * b[1] - a[1] * b[0]
        return {'AxB': f'({i}, {j}, {k})'}