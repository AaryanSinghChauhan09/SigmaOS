from sigma_core.interfaces.base_sovereign import ISovereign
from sigma_core.system.decorators import LoggingDecorator, ResilienceDecorator, MetricsDecorator, PrivacyDecorator
import threading

from ._base import SystemFactory

class SystemFactory:
    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(SystemFactory, cls).__new__(cls)
                cls._instance._registry = {}
        return cls._instance