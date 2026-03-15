# Generated method: SigmaMathema.chemistry_data
import math

class SigmaMathema:
    def chemistry_data(self, symbol):
        """Offline Periodic Table & Chemical Constants for NCERT/IIT."""
        elements = {'H': {'name': 'Hydrogen', 'mass': 1.008, 'atomic': 1}, 'He': {'name': 'Helium', 'mass': 4.0026, 'atomic': 2}, 'Li': {'name': 'Lithium', 'mass': 6.94, 'atomic': 3}, 'C': {'name': 'Carbon', 'mass': 12.011, 'atomic': 6}, 'N': {'name': 'Nitrogen', 'mass': 14.007, 'atomic': 7}, 'O': {'name': 'Oxygen', 'mass': 15.999, 'atomic': 8}, 'Na': {'name': 'Sodium', 'mass': 22.99, 'atomic': 11}, 'Mg': {'name': 'Magnesium', 'mass': 24.305, 'atomic': 12}, 'Al': {'name': 'Aluminum', 'mass': 26.982, 'atomic': 13}, 'Si': {'name': 'Silicon', 'mass': 28.085, 'atomic': 14}, 'P': {'name': 'Phosphorus', 'mass': 30.974, 'atomic': 15}, 'S': {'name': 'Sulfur', 'mass': 32.06, 'atomic': 16}, 'Cl': {'name': 'Chlorine', 'mass': 35.45, 'atomic': 17}, 'K': {'name': 'Potassium', 'mass': 39.098, 'atomic': 19}, 'Ca': {'name': 'Calcium', 'mass': 40.078, 'atomic': 20}, 'Fe': {'name': 'Iron', 'mass': 55.845, 'atomic': 26}, 'Cu': {'name': 'Copper', 'mass': 63.546, 'atomic': 29}, 'Zn': {'name': 'Zinc', 'mass': 65.38, 'atomic': 30}}
        return elements.get(symbol, {'error': 'Element not in local offline DB'})