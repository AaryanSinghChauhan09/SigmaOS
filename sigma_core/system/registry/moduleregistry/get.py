# Generated method: ModuleRegistry.get
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def get(self, name: str) -> Optional[Any]:
        """
            Retrieve a module by name
        
            Args:
                name: Module identifier
            
            Returns:
                Module instance or None if not found
            """
        with self._lock:
            return self._modules.get(name)