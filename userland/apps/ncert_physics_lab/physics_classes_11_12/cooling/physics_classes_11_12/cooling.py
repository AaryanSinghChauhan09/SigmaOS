# Generated method: Physics_Classes_11_12.cooling
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def cooling(te, to, k):
        t_seq = [0, 5, 10, 20, 30]
        res = {f'T at {t}m': _r(te + (to - te) * math.exp(-k * t), 1) for t in t_seq}
        return res