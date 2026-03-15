# Generated method: Chemistry_Classes_11_12.functional
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def functional(s):
        s = s.lower()
        if 'vinegar' in s or 'acetic' in s:
            return {'Group': '-COOH (Carboxylic Acid)', 'Test': 'Effervescence with NaHCO3'}
        if 'alcohol' in s or 'ethanol' in s:
            return {'Group': '-OH (Alcohol)', 'Test': 'Ester formation with Acid'}
        if 'acetone' in s:
            return {'Group': '>C=O (Ketone)', 'Test': 'Sodium Nitroprusside'}
        return {'Group': 'Unknown', 'Action': 'Perform Lucas Test'}