# Generated method: Physics_Classes_11_12.specific_heat
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def specific_heat(m, q, dt):
        return {'c (J/kg.K)': _r(q / (m * dt), 1)}