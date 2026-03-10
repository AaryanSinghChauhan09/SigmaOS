"""
SigmaOS Module Registry — hot-pluggable module management.
Modules register themselves; the kernel discovers them at runtime.
"""
import importlib
import threading
from typing import Any

class ModuleRegistry:
    """
    Central registry for all SigmaOS kernel & ecosystem modules.
    Supports hot-plug: modules can be added/removed without rebooting.
    """
    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super().__new__(cls)
                cls._instance._modules: dict[str, Any] = {}
                cls._instance._metadata: dict[str, dict] = {}
        return cls._instance

    def register(self, name: str, instance: Any, meta: dict | None = None):
        """Register a module instance under a string key."""
        self._modules[name] = instance
        self._metadata[name] = meta or {}
        return self

    def get(self, name: str) -> Any:
        return self._modules.get(name)

    def list_modules(self) -> list[str]:
        return list(self._modules.keys())

    def unregister(self, name: str) -> bool:
        if name in self._modules:
            del self._modules[name]
            del self._metadata[name]
            return True
        return False

    def get_meta(self, name: str) -> dict:
        return self._metadata.get(name, {})

    def call(self, module_name: str, method: str, *args, **kwargs) -> Any:
        """Remote-call: invoke a method on a registered module by name."""
        mod = self.get(module_name)
        if mod is None:
            return {"error": f"Module '{module_name}' not registered."}
        fn = getattr(mod, method, None)
        if fn is None:
            return {"error": f"Method '{method}' not found on '{module_name}'."}
        return fn(*args, **kwargs)

    def health_check(self) -> dict:
        results = {}
        for name, mod in self._modules.items():
            fn = getattr(mod, "health_check", None)
            results[name] = fn() if callable(fn) else "OK (no check)"
        return results
