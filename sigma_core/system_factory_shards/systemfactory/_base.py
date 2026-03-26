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