# Generated method: Chemistry_Classes_6_10.vsepr
import math, re

class Chemistry_Classes_6_10:
    @staticmethod
    def vsepr(s, l):
        s, l = (int(s), int(l))
        if s == 2:
            return {'Shape': 'Linear'}
        if s == 3:
            return {'Shape': 'Trigonal Planar' if l == 0 else 'Bent'}
        if s == 4:
            return {'Shape': 'Tetrahedral' if l == 0 else 'Trigonal Pyramidal' if l == 1 else 'Bent'}
        return {'Shape': 'Complex'}