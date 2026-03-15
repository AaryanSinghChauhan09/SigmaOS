# Generated method: Biology_Classes_11_12.hydrilla
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def hydrilla(l, t):
        bubbles = 10 if 'strong' in l.lower() else 2
        total = bubbles * (t / 10)
        return {'Oxygen Bubbles': int(total), 'Observation': 'Evolution of Gas'}