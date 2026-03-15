# Generated method: ModuleRegistry.__init__
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def __init__(self):
        """Initialize the module registry"""
        self._modules: Dict[str, Any] = {}
        self._metadata: Dict[str, Dict[str, Any]] = {}
        self._lock = threading.RLock()