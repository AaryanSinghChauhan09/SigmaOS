# Generated method: SigmaMathema.jee_derivative_sim
import math

class SigmaMathema:
    def jee_derivative_sim(self, function_str, x_val):
        """Simulation of Newtonian derivative for JEE Physics/Math."""
        h = 1e-07
        f = lambda x: self.evaluate_expression(function_str.replace('x', f'({x})'))
        if isinstance(f(x_val), str):
            return f(x_val)
        return (f(x_val + h) - f(x_val)) / h