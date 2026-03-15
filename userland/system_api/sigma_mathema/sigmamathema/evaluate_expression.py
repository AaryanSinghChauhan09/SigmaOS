# Generated method: SigmaMathema.evaluate_expression
import math

class SigmaMathema:
    def evaluate_expression(self, expr: str):
        """Safely evaluates math expressions including JEE-level calculus/trig."""
        try:
            safe_namespace = {'sin': math.sin, 'cos': math.cos, 'tan': math.tan, 'asin': math.asin, 'acos': math.acos, 'atan': math.atan, 'sinh': math.sinh, 'cosh': math.cosh, 'tanh': math.tanh, 'log': math.log10, 'ln': math.log, 'sqrt': math.sqrt, 'exp': math.exp, 'pi': math.pi, 'e': math.e, 'factorial': math.factorial, 'pow': pow, 'abs': abs, 'round': round, 'log2': math.log2, 'gamma': math.gamma, 'ceil': math.ceil, 'floor': math.floor}
            result = eval(expr, {'__builtins__': None}, safe_namespace)
            self.history.append((expr, result))
            return result
        except Exception as e:
            return f'Error: {str(e)}'