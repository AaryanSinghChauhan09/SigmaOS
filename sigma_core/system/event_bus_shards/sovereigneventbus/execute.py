from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading

from ._base import SovereignEventBus

class SovereignEventBus:
    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract."""
        if action == 'LIST_TOPICS':
            return list(self._subscribers.keys())
        return None