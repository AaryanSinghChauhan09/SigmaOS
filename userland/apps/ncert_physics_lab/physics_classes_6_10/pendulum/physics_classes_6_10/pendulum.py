# Generated method: Physics_Classes_6_10.pendulum
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def pendulum(l):
        return {'Period T (s)': _r(2 * math.pi * math.sqrt(l / 9.81), 3)}