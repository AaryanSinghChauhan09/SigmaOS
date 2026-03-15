# Generated method: SigmaMathema.molar_mass_calc
import math

class SigmaMathema:
    def molar_mass_calc(self, composition):
        """Calculates molar mass for JEE Stoichiometry. Input: {'H': 2, 'O': 1}"""
        total = 0
        for sym, count in composition.items():
            data = self.chemistry_data(sym)
            if 'error' in data:
                return data
            total += data['mass'] * count
        return total