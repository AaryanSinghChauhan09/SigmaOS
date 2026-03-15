# Generated method: SigmaMathema.jee_integral_sim
import math

class SigmaMathema:
    def jee_integral_sim(self, function_str, lower, upper):
        """Simpson's rule integration for Class 12 Calculus."""
        n = 1000
        dx = (upper - lower) / n
        total = 0
        f = lambda x: self.evaluate_expression(function_str.replace('x', f'({x})'))
        for i in range(n):
            x = lower + i * dx
            res = f(x)
            if isinstance(res, str):
                return res
            total += res * dx
        return total