from .interfaces.system_interfaces import ISystemComponent
import threading

class SystemFactory:
    """
    Singleton & Abstract Factory Pattern.
    Manage and inject system-wide singletons.
    """
    _instance = None
    _lock = threading.Lock()