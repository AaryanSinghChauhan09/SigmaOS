# Generated method: SigmaModularEngine.dynamic_purge
import os

class SigmaModularEngine:
    def dynamic_purge(self, module_name):
        """Removes a module to reduce attack surface or free up system resources."""
        if module_name in self.activated_modules:
            self.activated_modules.remove(module_name)
            del self.compliance_matrix[module_name]
            return f'Purge: {module_name} successfully deactivated and scrubbed from memory.'
        return 'Error: Module not found.'