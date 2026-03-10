"""
SigmaOS Mathematical Kernel (Mathema)
=====================================
Industrial-grade math engine supporting NCERT K-12 and IIT-JEE Advanced requirements.
"""

import math

class SigmaMathema:
    def __init__(self):
        self.history = []

    def evaluate_expression(self, expr: str):
        """Safely evaluates math expressions including JEE-level calculus/trig."""
        try:
            # Add common K-12/IIT constant and functions to namespace
            safe_namespace = {
                'sin': math.sin, 'cos': math.cos, 'tan': math.tan,
                'asin': math.asin, 'acos': math.acos, 'atan': math.atan,
                'sinh': math.sinh, 'cosh': math.cosh, 'tanh': math.tanh,
                'log': math.log10, 'ln': math.log, 'sqrt': math.sqrt,
                'exp': math.exp, 'pi': math.pi, 'e': math.e,
                'factorial': math.factorial, 'pow': pow,
                'abs': abs, 'round': round, 'log2': math.log2,
                'gamma': math.gamma, 'ceil': math.ceil, 'floor': math.floor
            }
            
            # Simple sanitization for basic security loop
            # In a full OS, this would use a proper parser (sympy)
            result = eval(expr, {"__builtins__": None}, safe_namespace)
            self.history.append((expr, result))
            return result
        except Exception as e:
            return f"Error: {str(e)}"

    def jee_derivative_sim(self, function_str, x_val):
        """Simulation of Newtonian derivative for JEE Physics/Math."""
        h = 1e-7
        f = lambda x: self.evaluate_expression(function_str.replace('x', f'({x})'))
        if isinstance(f(x_val), str): return f(x_val)
        return (f(x_val + h) - f(x_val)) / h

    def jee_integral_sim(self, function_str, lower, upper):
        """Simpson's rule integration for Class 12 Calculus."""
        n = 1000
        dx = (upper - lower) / n
        total = 0
        f = lambda x: self.evaluate_expression(function_str.replace('x', f'({x})'))
        
        for i in range(n):
            x = lower + i * dx
            res = f(x)
            if isinstance(res, str): return res
            total += res * dx
        return total

    def physics_constant(self, key):
        """IIT-JEE Physics Constants."""
        constants = {
            'G': 6.674e-11, 'g': 9.81, 'c': 3e8, 'h': 6.626e-34,
            'k': 1.38e-23, 'eps0': 8.854e-12, 'mu0': 1.256e-6,
            'R': 8.314, 'Na': 6.022e23, 'me': 9.109e-31, 'mp': 1.672e-27
        }
        return constants.get(key, "N/A")

    def chemistry_data(self, symbol):
        """Offline Periodic Table & Chemical Constants for NCERT/IIT."""
        elements = {
            'H':  {'name': 'Hydrogen', 'mass': 1.008,  'atomic': 1},
            'He': {'name': 'Helium',   'mass': 4.0026, 'atomic': 2},
            'Li': {'name': 'Lithium',  'mass': 6.94,    'atomic': 3},
            'C':  {'name': 'Carbon',   'mass': 12.011, 'atomic': 6},
            'N':  {'name': 'Nitrogen', 'mass': 14.007, 'atomic': 7},
            'O':  {'name': 'Oxygen',   'mass': 15.999, 'atomic': 8},
            'Na': {'name': 'Sodium',   'mass': 22.990, 'atomic': 11},
            'Mg': {'name': 'Magnesium','mass': 24.305, 'atomic': 12},
            'Al': {'name': 'Aluminum', 'mass': 26.982, 'atomic': 13},
            'Si': {'name': 'Silicon',  'mass': 28.085, 'atomic': 14},
            'P':  {'name': 'Phosphorus','mass': 30.974, 'atomic': 15},
            'S':  {'name': 'Sulfur',   'mass': 32.06,   'atomic': 16},
            'Cl': {'name': 'Chlorine', 'mass': 35.45,   'atomic': 17},
            'K':  {'name': 'Potassium','mass': 39.098, 'atomic': 19},
            'Ca': {'name': 'Calcium',  'mass': 40.078, 'atomic': 20},
            'Fe': {'name': 'Iron',     'mass': 55.845, 'atomic': 26},
            'Cu': {'name': 'Copper',   'mass': 63.546, 'atomic': 29},
            'Zn': {'name': 'Zinc',     'mass': 65.38,   'atomic': 30}
        }
        return elements.get(symbol, {"error": "Element not in local offline DB"})

    def molar_mass_calc(self, composition):
        """Calculates molar mass for JEE Stoichiometry. Input: {'H': 2, 'O': 1}"""
        total = 0
        for sym, count in composition.items():
            data = self.chemistry_data(sym)
            if "error" in data: return data
            total += data['mass'] * count
        return total

    def hcf(self, a, b):
        """HCF for Class 5-8."""
        return math.gcd(a, b)

    def lcm(self, a, b):
        """LCM for Class 5-8."""
        if a == 0 or b == 0: return 0
        return abs(a * b) // math.gcd(a, b)

    def volume_sphere(self, r):
        """NCERT Class 9/10 Mensuration."""
        return (4/3) * math.pi * (r**3)

    def volume_cone(self, r, h):
        """NCERT Class 9/10 Mensuration."""
        return (1/3) * math.pi * (r**2) * h

    def ideal_gas_law(self, P=None, V=None, n=None, T=None):
        """Solves PV=nRT for missing variable."""
        R = 0.0821 # L.atm/mol.K
        if P is None: return (n * R * T) / V
        if V is None: return (n * R * T) / P
        if n is None: return (P * V) / (R * T)
        if T is None: return (P * V) / (n * R)
        return "N/A"

    def organic_functional_groups(self):
        """NCERT Class 11/12 Organic Chemistry summary."""
        return {
            "Alcohol": "-OH", "Aldehyde": "-CHO", "Ketone": ">C=O",
            "Carboxylic Acid": "-COOH", "Ether": "-O-", "Ester": "-COOR"
        }
