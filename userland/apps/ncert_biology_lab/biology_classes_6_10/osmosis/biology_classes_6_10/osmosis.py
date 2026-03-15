# Generated method: Biology_Classes_6_10.osmosis
import math, random

class Biology_Classes_6_10:
    @staticmethod
    def osmosis(cc, sc):
        if sc > cc:
            return {'Process': 'Exosmosis'}
        if sc < cc:
            return {'Process': 'Endosmosis'}
        return {'Process': 'Equil'}