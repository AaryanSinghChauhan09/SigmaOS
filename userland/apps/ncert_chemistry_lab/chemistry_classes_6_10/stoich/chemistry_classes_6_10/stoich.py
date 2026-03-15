# Generated method: Chemistry_Classes_6_10.stoich
import math, re

class Chemistry_Classes_6_10:
    @staticmethod
    def stoich(m, mmr, mmp):
        return {'Product Mass': _r(m / mmr * mmp, 2)}