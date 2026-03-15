# Generated method: Biology_Classes_11_12.mitosis
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def mitosis(p):
        d = {'metaphase': 'Aligned at Equator', 'anaphase': 'Separating'}
        return {'Obs': d.get(p.lower(), 'Division Stage')}