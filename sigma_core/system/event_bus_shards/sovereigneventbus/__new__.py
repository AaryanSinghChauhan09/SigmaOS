from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading

from ._base import SovereignEventBus

class SovereignEventBus:
    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(SovereignEventBus, cls).__new__(cls)
                cls._instance._subscribers = {}
        return cls._instance