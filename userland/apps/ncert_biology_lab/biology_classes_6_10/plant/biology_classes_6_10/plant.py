# Generated method: Biology_Classes_6_10.plant
import math, random

class Biology_Classes_6_10:
    @staticmethod
    def plant(l, r):
        if 'dicot' in l.lower() or 'tap' in r.lower():
            return {'Type': 'DICOT'}
        return {'Type': 'MONOCOT'}