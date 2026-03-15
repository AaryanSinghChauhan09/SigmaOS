# Generated method: ModuleRegistry.register
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def register(self, name: str, module: Any, metadata: Dict[str, Any]=None) -> None:
        """
            Register a module in the registry
        
            Args:
                name: Unique module identifier
                module: Module instance or class
                metadata: Optional metadata (source, class name, version, etc.)
            """
        with self._lock:
            self._modules[name] = module
            self._metadata[name] = metadata or {'class': type(module).__name__, 'source': 'kernel'}