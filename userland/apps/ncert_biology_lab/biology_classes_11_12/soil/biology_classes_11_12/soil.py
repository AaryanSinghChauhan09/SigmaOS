# Generated method: Biology_Classes_11_12.soil
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def soil(w1, w2):
        moisture = (w1 - w2) / w1 * 100
        return {'Moisture %': _r(moisture, 2), 'Type': 'Loamy' if 10 < moisture < 20 else 'Sandy/Clay'}