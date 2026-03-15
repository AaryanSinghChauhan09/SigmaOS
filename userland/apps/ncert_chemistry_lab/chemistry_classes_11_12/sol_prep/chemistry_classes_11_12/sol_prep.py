# Generated method: Chemistry_Classes_11_12.sol_prep
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def sol_prep(t):
        if 'gold' in t.lower():
            return {'Method': "Bredig's Arc", 'Status': 'Purple Sol'}
        if 'ferric' in t.lower():
            return {'Method': 'Hydrolysis', 'Result': 'Reddish Brown Sol'}
        return {'Method': 'Peptization', 'Info': 'Add electrolyte to precipitate'}