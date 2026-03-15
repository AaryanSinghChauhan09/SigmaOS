# Generated method: EventBus.subscribe
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def subscribe(self, event_name: str, callback: Callable) -> None:
        """Subscribe to an event (O(1) write)."""
        with self._lock:
            if event_name not in self._subscribers:
                self._subscribers[event_name] = []
            self._subscribers[event_name].append(callback)