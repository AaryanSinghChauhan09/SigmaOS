# Generated method: EventBus.get_history
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def get_history(self, event_name: str | None=None) -> List[Any]:
        """Returns the event history; filters results to exclude 'None' slots."""
        with self._lock:
            h = [e for e in self._event_history if e is not None]
            if event_name:
                return [e for e in h if e['event'] == event_name]
            return h