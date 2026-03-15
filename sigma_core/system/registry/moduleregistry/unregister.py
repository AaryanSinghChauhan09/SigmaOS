# Generated method: ModuleRegistry.unregister
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def unregister(self, name: str) -> bool:
        """
            Unregister a module
        
            Args:
                name: Module identifier
            
            Returns:
                True if module was found and unregistered
            """
        with self._lock:
            if name in self._modules:
                del self._modules[name]
                del self._metadata[name]
                return True
            return False