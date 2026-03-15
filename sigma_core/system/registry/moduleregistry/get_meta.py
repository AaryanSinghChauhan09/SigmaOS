# Generated method: ModuleRegistry.get_meta
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def get_meta(self, name: str) -> Dict[str, Any]:
        """
            Get metadata for a registered module
        
            Args:
                name: Module identifier
            
            Returns:
                Metadata dictionary
            """
        with self._lock:
            return self._metadata.get(name, {})