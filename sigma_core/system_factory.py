from .interfaces.base_sovereign import ISovereign
from .system.decorators import LoggingDecorator, ResilienceDecorator
import threading

class SystemFactory:
    _instance = None
    _lock = threading.Lock()
    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(SystemFactory, cls).__new__(cls)
                cls._instance._registry = {}
        return cls._instance
    def register(self, name, component, resilient=True, logged=True):
        wrapped = component
        if resilient: wrapped = ResilienceDecorator(wrapped)
        if logged: wrapped = LoggingDecorator(wrapped)
        self._registry[name] = wrapped
    def get(self, name):
        return self._registry[name]

def get_factory(): return SystemFactory()
