# Generated method: EventBus.clear_history
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def clear_history(self) -> None:
        """Resets the history pointer and purges the ring buffer."""
        with self._lock:
            self._event_history = [None] * self._max_history
            self._history_ptr = 0