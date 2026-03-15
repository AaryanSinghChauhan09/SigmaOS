# Generated method: Chemistry_Classes_11_12.bond
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def bond(br, bp):
        dH = br - bp
        return {'delta_H': dH, 'Type': 'EXO' if dH < 0 else 'ENDO'}