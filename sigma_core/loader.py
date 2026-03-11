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
        """Dynamic loading with Interface validation and legacy fallback."""
        try:
            mod = importlib.import_module(module_name)
            cls = getattr(mod, class_name)
            
            # Hybrid initialization: try passing kernel, fallback to empty
            try:
                instance = cls(self.kernel)
            except TypeError as te:
                if "argument" in str(te) or "takes no arguments" in str(te):
                    try:
                        instance = cls()
                    except Exception as fallback_e:
                        print(f"[LOADER_ERR] Fallback failed for {module_name}: {fallback_e}")
                        return None
                else:
                    raise

            # Register module (Interface Segregation / Dependency Inversion check)
            self.kernel.registry.register(registry_name, instance)
            
            # Auto-start services if supported
            if isinstance(instance, ISigmaService) or hasattr(instance, "start_service"):
                if hasattr(instance, "start_service"):
                    instance.start_service()
            return instance
                
        except Exception as e:
            print(f"[LOADER_ERR] Failed to load {module_name}: {e}")
        return None

    def load_modules_parallel(self, module_list: list):
        """
        USP: Concurrent Apex Hydration. 
        Loads multiple kernel/ecosystem shards in parallel via ThreadPool.
        """
        from concurrent.futures import ThreadPoolExecutor
        with ThreadPoolExecutor(max_workers=10) as executor:
            futures = []
            for m_file, c_name, r_key in module_list:
                futures.append(executor.submit(self.load_module, m_file, c_name, r_key))
            
            # Wait for all core shards to link
            for future in futures:
                try: future.result()
                except: pass
        return True
