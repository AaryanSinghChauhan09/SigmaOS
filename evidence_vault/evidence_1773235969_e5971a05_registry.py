"""
SigmaOS Module Registry
Manages dynamic registration and lifecycle of system modules.
"""

from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    """
    Central registry for SigmaOS modules.
    Handles registration, lookup, health checks, and module lifecycle.
    """
    
    def __init__(self):
        """Initialize the module registry"""
        self._modules: Dict[str, Any] = {}
        self._metadata: Dict[str, Dict[str, Any]] = {}
        self._lock = threading.RLock()
    
    def register(self, name: str, module: Any, metadata: Dict[str, Any] = None) -> None:
        """
        Register a module in the registry
        
        Args:
            name: Unique module identifier
            module: Module instance or class
            metadata: Optional metadata (source, class name, version, etc.)
        """
        with self._lock:
            self._modules[name] = module
            self._metadata[name] = metadata or {
                "class": type(module).__name__,
                "source": "kernel"
            }
    
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
    
    def list_modules(self) -> List[str]:
        """
        Get list of all registered module names
        
        Returns:
            List of module names
        """
        with self._lock:
            return list(self._modules.keys())
    
    def exists(self, name: str) -> bool:
        """Check if a module is registered"""
        with self._lock:
            return name in self._modules
    
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
    
    def health_check(self) -> Dict[str, str]:
        """
        Perform health check on all registered modules
        
        Returns:
            Dictionary mapping module names to health status
        """
        health = {}
        with self._lock:
            for name, module in self._modules.items():
                if hasattr(module, 'health_check'):
                    try:
                        health[name] = module.health_check()
                    except Exception as e:
                        health[name] = f"ERROR: {str(e)}"
                else:
                    health[name] = "OK"
        return health
