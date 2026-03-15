from .interfaces.base_sovereign import ISovereign
from .system.decorators import LoggingDecorator, ResilienceDecorator
import threading

class SystemFactory:
    """
    Sovereign Factory (Singleton).
    Manages Shard Creation, Decoration, and Dependency Injection.
    """
    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(SystemFactory, cls).__new__(cls)
                # Initialize registry on the instance
                cls._instance._registry = {}
        return cls._instance

    def register(self, name: str, component: ISovereign, resilient: bool = True, logged: bool = True):
        """
        Registers and provides a decorated Sovereign Unit.
        """
        wrapped = component
        if resilient:
            wrapped = ResilienceDecorator(wrapped)
        if logged:
            wrapped = LoggingDecorator(wrapped)
            
        print(f"[FACTORY] Assembling Sovereign Unit: {name}")
        self._registry[name] = wrapped

    def get(self, name: str) -> ISovereign:
        """Retrieves an assembled Sovereign Unit."""
        if name not in self._registry:
             raise KeyError(f"Sovereign Unit '{name}' not found.")
        return self._registry[name]

def get_factory() -> SystemFactory:
    return SystemFactory()
