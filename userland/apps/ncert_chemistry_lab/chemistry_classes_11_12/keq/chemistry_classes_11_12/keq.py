# Generated method: Chemistry_Classes_11_12.keq
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def keq(kc, qc):
        if qc < kc:
            return {'Shift': 'FORWARD'}
        if qc > kc:
            return {'Shift': 'BACKWARD'}
        return {'Shift': 'EQUILIBRIUM'}