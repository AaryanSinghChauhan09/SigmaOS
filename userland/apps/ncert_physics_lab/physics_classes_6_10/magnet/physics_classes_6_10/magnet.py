# Generated method: Physics_Classes_6_10.magnet
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def magnet(p1, p2):
        if p1.upper() == p2.upper():
            return {'Result': 'REPEL'}
        return {'Result': 'ATTRACT'}