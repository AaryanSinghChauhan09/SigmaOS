# Generated method: ModuleRegistry.list_modules
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def list_modules(self) -> List[str]:
        """
            Get list of all registered module names
        
            Returns:
                List of module names
            """
        with self._lock:
            return list(self._modules.keys())