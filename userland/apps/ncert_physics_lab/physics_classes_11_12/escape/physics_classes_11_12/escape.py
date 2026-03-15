# Generated method: Physics_Classes_11_12.escape
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def escape(m, r_km):
        g = 6.67e-11
        r = r_km * 1000
        v = math.sqrt(2 * g * m / r)
        return {'v_esc (m/s)': _r(v, 1)}