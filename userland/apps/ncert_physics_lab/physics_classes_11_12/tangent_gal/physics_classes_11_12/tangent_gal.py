# Generated method: Physics_Classes_11_12.tangent_gal
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def tangent_gal(th, n, r):
        mu0 = 4 * math.pi * 1e-07
        r_m = r / 100
        bh = 3.5e-05
        i = 2 * r_m * bh * math.tan(math.radians(th)) / (mu0 * n)
        return {'Current I (A)': _r(i, 4), 'Bh Used (T)': bh}