# Generated method: Physics_Classes_11_12.torque
import math, random

class Physics_Classes_11_12:
    @staticmethod
    def torque(b, a, i, th):
        t = i * a * b * math.sin(math.radians(th))
        return {'Torque (Nm)': _r(t, 4)}