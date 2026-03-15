# Generated method: SigmaModularEngine.get_module_compliance
import os

class SigmaModularEngine:
    def get_module_compliance(self, module_name):
        """Returns the specific standards a module is certified against."""
        return self.compliance_matrix.get(module_name, 'UNVERIFIED')