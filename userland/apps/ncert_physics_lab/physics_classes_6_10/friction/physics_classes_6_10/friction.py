# Generated method: Physics_Classes_6_10.friction
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def friction(m, s):
        u = {'ice': 0.05, 'wood': 0.3, 'rubber': 0.7}.get(s.lower(), 0.3)
        return {'Friction (N)': _r(m * 9.81 * u, 2), 'Mu': u}