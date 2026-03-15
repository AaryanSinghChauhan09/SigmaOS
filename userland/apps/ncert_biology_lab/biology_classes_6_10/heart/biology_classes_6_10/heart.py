# Generated method: Biology_Classes_6_10.heart
import math, random

class Biology_Classes_6_10:
    @staticmethod
    def heart(age, ex):
        m = 220 - age
        return {'bpm': 110 if ex else 72, 'Max': m}