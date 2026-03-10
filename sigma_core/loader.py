"""
SigmaOS Module Loader (SOLID Implementation)
=============================================
Follows the Single Responsibility Principle by decoupling module initialization 
from the core kernel logic.
"""

import importlib
from .interfaces import ISigmaModule, ISigmaService

class SigmaModuleLoader:
    def __init__(self, kernel):
        self.kernel = kernel

    def load_module(self, module_name: str, class_name: str, registry_name: str):
        """Dynamic loading with Interface validation."""
        try:
            mod = importlib.import_module(module_name)
            cls = getattr(mod, class_name)
            instance = cls(self.kernel)
            
            # Interface Segregation / Dependency Inversion check
            if isinstance(instance, ISigmaModule):
                self.kernel.registry.register(registry_name, instance)
                
                # Auto-start services (LSP)
                if isinstance(instance, ISigmaService):
                    instance.start_service()
                
                return instance
            else:
                print(f"[LOADER_ERR] {class_name} does not implement ISigmaModule.")
        except Exception as e:
            print(f"[LOADER_ERR] Failed to load {module_name}: {e}")
        return None
