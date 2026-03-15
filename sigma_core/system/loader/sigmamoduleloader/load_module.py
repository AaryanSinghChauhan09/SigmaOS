# Generated method: SigmaModuleLoader.load_module
import importlib
from .interfaces import ISigmaModule, ISigmaService

class SigmaModuleLoader:
    def load_module(self, module_name: str, class_name: str, registry_name: str):
        """Dynamic loading with Interface validation and legacy fallback."""
        try:
            mod = importlib.import_module(module_name)
            cls = getattr(mod, class_name)
            try:
                instance = cls(self.kernel)
            except TypeError as te:
                if 'argument' in str(te) or 'takes no arguments' in str(te):
                    try:
                        instance = cls()
                    except Exception as fallback_e:
                        print(f'[LOADER_ERR] Fallback failed for {module_name}: {fallback_e}')
                        return None
                else:
                    raise
            self.kernel.registry.register(registry_name, instance)
            if isinstance(instance, ISigmaService) or hasattr(instance, 'start_service'):
                if hasattr(instance, 'start_service'):
                    instance.start_service()
            return instance
        except Exception as e:
            print(f'[LOADER_ERR] Failed to load {module_name}: {e}')
        return None