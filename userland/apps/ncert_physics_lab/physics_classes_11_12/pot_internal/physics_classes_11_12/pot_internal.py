# Generated method: Physics_Classes_11_12.pot_internal
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def pot_internal(l1, l2, r):
        int_r = r * (l1 - l2) / l2
        return {'Internal r (Ω)': _r(int_r, 2)}