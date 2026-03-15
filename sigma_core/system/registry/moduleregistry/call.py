# Generated method: ModuleRegistry.call
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def call(self, module_name: str, method_name: str, *args, **kwargs) -> Any:
        """
            Call a method on a registered module
        
            Args:
                module_name: Module identifier
                method_name: Method name to call
                *args, **kwargs: Arguments to pass to method
            
            Returns:
                Result of method call or None
            """
        module = self.get(module_name)
        if module and hasattr(module, method_name):
            method = getattr(module, method_name)
            if callable(method):
                return method(*args, **kwargs)
        return None