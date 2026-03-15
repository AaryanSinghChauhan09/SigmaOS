# Generated method: Biology_Classes_11_12.lung
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def lung(tv, irv, erv):
        vc = tv + irv + erv
        return {'Vital Capacity (ml)': vc, 'IC (Inspiratory)': tv + irv}