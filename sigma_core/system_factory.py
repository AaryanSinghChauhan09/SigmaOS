from sigma_core.interfaces.base_sovereign import ISovereign
from sigma_core.system.decorators import LoggingDecorator, ResilienceDecorator, MetricsDecorator, PrivacyDecorator
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

    def register(self, name: str, component: ISovereign, resilient: bool = True, logged: bool = True, metrics: bool = True):
        """
        Registers and provides a decorated Sovereign Unit.
        """
        wrapped = component
        if resilient:
            wrapped = ResilienceDecorator(wrapped)
        if metrics:
            wrapped = MetricsDecorator(wrapped)
        if logged:
            wrapped = LoggingDecorator(wrapped)
            
        # Mandatory Privacy Proxy for Shards (Zero Trust)
        privacy_guard = self._registry.get("PrivacyGuard")
        if privacy_guard and name != "PrivacyGuard":
            # Extract required tag from component metadata if available, else default to name
            tag = getattr(component, 'privacy_tag', name)
            wrapped = PrivacyDecorator(wrapped, privacy_guard, tag)
            
        print(f"[FACTORY] Assembling Sovereign Unit: {name}")
        self._registry[name] = wrapped

    def get(self, name: str) -> ISovereign:
        """Retrieves an assembled Sovereign Unit."""
        if name not in self._registry:
             raise KeyError(f"Sovereign Unit '{name}' not found.")
        return self._registry[name]

def get_factory() -> SystemFactory:
    return SystemFactory()
