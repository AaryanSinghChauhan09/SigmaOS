# Generated method: Physics_Classes_11_12.resonance
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def resonance(f, l1_cm):
        l1 = l1_cm / 100
        v = 4 * f * (l1 + 0.3 * (2 * 0.02))
        return {'Speed of Sound (m/s)': _r(v, 2)}