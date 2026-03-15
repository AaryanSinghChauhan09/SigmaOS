# Generated method: Physics_Classes_6_10.eye
import math, random

class Physics_Classes_6_10:
    @staticmethod
    def eye(d):
        if d < 0.25:
            return {'Status': 'Blurry', 'Reason': 'Near point limit (25cm)'}
        p = 1 / d
        return {'Lens Power (D)': _r(p, 2), 'Acommodation': 'Active'}