from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading

from ._base import SovereignEventBus

class SovereignEventBus:
    def publish(self, event_type: str, data: dict):
        print(f'[EVENT_BUS] Publishing: {event_type}')
        if event_type in self._subscribers:
            for observer in self._subscribers[event_type]:
                try:
                    observer.on_event(event_type, data)
                except Exception as e:
                    print(f'[EVENT_BUS-ERROR] Observer failure: {e}')