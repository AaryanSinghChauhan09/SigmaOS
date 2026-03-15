# Generated method: SigmaMathema.lcm
import math

class SigmaMathema:
    def lcm(self, a, b):
        """LCM for Class 5-8."""
        if a == 0 or b == 0:
            return 0
        return abs(a * b) // math.gcd(a, b)