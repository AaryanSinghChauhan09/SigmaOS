# Generated method: SigmaMathema.physics_constant
import math

class SigmaMathema:
    def physics_constant(self, key):
        """IIT-JEE Physics Constants."""
        constants = {'G': 6.674e-11, 'g': 9.81, 'c': 300000000.0, 'h': 6.626e-34, 'k': 1.38e-23, 'eps0': 8.854e-12, 'mu0': 1.256e-06, 'R': 8.314, 'Na': 6.022e+23, 'me': 9.109e-31, 'mp': 1.672e-27}
        return constants.get(key, 'N/A')