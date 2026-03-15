# Generated method: ModuleRegistry.exists
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def exists(self, name: str) -> bool:
        """Check if a module is registered"""
        with self._lock:
            return name in self._modules