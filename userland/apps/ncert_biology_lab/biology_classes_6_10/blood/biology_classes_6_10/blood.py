# Generated method: Biology_Classes_6_10.blood
import math, random

class Biology_Classes_6_10:
    @staticmethod
    def blood(a, b, rh):
        g = 'O'
        if a and b:
            g = 'AB'
        elif a:
            g = 'A'
        elif b:
            g = 'B'
        return {'Group': g + ('+' if rh else '-')}