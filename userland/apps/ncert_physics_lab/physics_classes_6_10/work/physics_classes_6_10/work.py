# Generated method: Physics_Classes_6_10.work
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def work(f, d, a):
        w = f * d * math.cos(math.radians(a))
        return {'Work (J)': _r(w, 2)}