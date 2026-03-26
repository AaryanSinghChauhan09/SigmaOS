from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading

from ._base import SovereignEventBus

class SovereignEventBus:
    def health_check(self) -> bool:
        return True