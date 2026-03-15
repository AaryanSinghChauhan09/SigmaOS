# Generated method: SigmaMathema.ideal_gas_law
import math

class SigmaMathema:
    def ideal_gas_law(self, P=None, V=None, n=None, T=None):
        """Solves PV=nRT for missing variable."""
        R = 0.0821
        if P is None:
            return n * R * T / V
        if V is None:
            return n * R * T / P
        if n is None:
            return P * V / (R * T)
        if T is None:
            return P * V / (n * R)
        return 'N/A'