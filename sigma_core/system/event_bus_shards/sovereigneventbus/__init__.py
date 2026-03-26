from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading

from ._base import SovereignEventBus

class SovereignEventBus:
    def __init__(self):
        if not hasattr(self, 'name'):
            super().__init__('EVENT_BUS')