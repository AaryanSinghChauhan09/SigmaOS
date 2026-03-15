# Generated method: Chemistry_Classes_6_10.ph
import math, re

class Chemistry_Classes_6_10:
    @staticmethod
    def ph(c, isa):
        p = -math.log10(c) if int(isa) else 14 + math.log10(c)
        return {'pH': _r(p, 2), 'Nature': 'Acid' if p < 7 else 'Base'}