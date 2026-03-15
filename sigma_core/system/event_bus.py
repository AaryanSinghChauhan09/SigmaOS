from ..interfaces.base_sovereign import SovereignModule
from ..interfaces.event_interfaces import IEventBus, IEventObserver
import threading

class SovereignEventBus(SovereignModule, IEventBus):
    """
    Sovereign Event Bus (Singleton).
    Mediates communication between decoupled shards.
    """
    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(SovereignEventBus, cls).__new__(cls)
                cls._instance._subscribers = {}
        return cls._instance

    def __init__(self):
        if not hasattr(self, 'name'):
            super().__init__("EVENT_BUS")

    def subscribe(self, event_type: str, observer: IEventObserver):
        print(f"[EVENT_BUS] New subscription for: {event_type}")
        if event_type not in self._subscribers:
            self._subscribers[event_type] = []
        self._subscribers[event_type].append(observer)

    def publish(self, event_type: str, data: dict):
        print(f"[EVENT_BUS] Publishing: {event_type}")
        if event_type in self._subscribers:
            for observer in self._subscribers[event_type]:
                try:
                    observer.on_event(event_type, data)
                except Exception as e:
                    print(f"[EVENT_BUS-ERROR] Observer failure: {e}")

    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract."""
        if action == "LIST_TOPICS":
            return list(self._subscribers.keys())
        return None

    def initialize(self):
        print("[EVENT_BUS] Event System Online.")

    def shutdown(self):
        self._subscribers.clear()
        print("[EVENT_BUS] Event System Offline.")

    def health_check(self) -> bool:
        return True

def get_event_bus() -> SovereignEventBus:
    return SovereignEventBus()
