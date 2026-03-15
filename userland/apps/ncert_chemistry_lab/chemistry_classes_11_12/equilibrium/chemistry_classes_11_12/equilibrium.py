# Generated method: Chemistry_Classes_11_12.equilibrium
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def equilibrium(a, b, c):
        kc = c / (a * b)
        return {'Kc': _r(kc, 2), 'Prediction': 'Stable' if kc > 1 else 'Reactants Favored'}