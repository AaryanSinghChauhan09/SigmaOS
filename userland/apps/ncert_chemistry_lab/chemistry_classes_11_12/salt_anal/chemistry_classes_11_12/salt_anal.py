# Generated method: Chemistry_Classes_11_12.salt_anal
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def salt_anal(c, f):
        c, f = (c.lower(), f.lower())
        if 'blue' in c or 'green' in f:
            return {'Cation': 'Cu2+', 'Confirmation': 'Deep blue with Ammonia'}
        if 'brick red' in f:
            return {'Cation': 'Ca2+', 'Confirmation': 'White ppt with Ammonium Oxalate'}
        if 'white' in c and 'apple green' in f:
            return {'Cation': 'Ba2+', 'Confirmation': 'Yellow ppt with K2CrO4'}
        return {'Cation': 'Needs Wet Test', 'Action': 'Add NaOH'}