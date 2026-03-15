# Generated method: EventBus.unsubscribe
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def unsubscribe(self, event_name: str, callback: Callable) -> None:
        """Unsubscribe from an event."""
        with self._lock:
            if event_name in self._subscribers:
                if callback in self._subscribers[event_name]:
                    self._subscribers[event_name].remove(callback)