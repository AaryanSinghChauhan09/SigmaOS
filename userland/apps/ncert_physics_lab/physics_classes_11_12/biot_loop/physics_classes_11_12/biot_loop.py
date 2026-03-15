# Generated method: Physics_Classes_11_12.biot_loop
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def biot_loop(i, r, z):
        mu0 = 4 * math.pi * 1e-07
        r /= 100
        z /= 100
        bz = mu0 * i * r ** 2 / (2 * (r ** 2 + z ** 2) ** 1.5)
        return {'B (Tesla)': f'{bz:.4e}'}