# Generated method: Physics_Classes_6_10.snell
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def snell(n1, th, n2):
        r1 = math.radians(th)
        s2 = n1 * math.sin(r1) / n2
        if s2 > 1:
            return {'Result': 'TIR'}
        return {'r Angle': _r(math.degrees(math.asin(s2)), 2)}