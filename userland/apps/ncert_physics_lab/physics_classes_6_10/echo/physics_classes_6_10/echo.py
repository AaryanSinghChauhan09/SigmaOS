# Generated method: Physics_Classes_6_10.echo
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def echo(t, temp):
        v = 331 + 0.6 * temp
        d = v * t / 2
        return {'Distance to Obstacle (m)': _r(d, 2), 'Min Distance for Echo': '17.2m (at 20C)'}