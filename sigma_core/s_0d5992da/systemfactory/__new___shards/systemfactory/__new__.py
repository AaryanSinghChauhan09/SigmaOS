from .interfaces.system_interfaces import ISystemComponent
import threading

class SystemFactory:
    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(SystemFactory, cls).__new__(cls)
                cls._instance._registry = {}
        return cls._instance