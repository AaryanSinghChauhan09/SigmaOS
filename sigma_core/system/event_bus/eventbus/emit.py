# Generated method: EventBus.emit
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def emit(self, event_name: str, data: Any=None, synchronous: bool=False, priority: int=2) -> None:
        """
            Emit an event via the FAST-PATH or the WORKER-PATH.
            """
        interaction_boost = self._interaction_weights.get(event_name, 1.0)
        is_critical = priority <= 1 or 'fault' in event_name or 'kernel' in event_name or (interaction_boost > 2.0)
        if interaction_boost > 1.0:
            self._interaction_weights[event_name] = min(interaction_boost + 0.1, 5.0)
        else:
            self._interaction_weights[event_name] = 1.1
        with self._lock:
            self._event_history[self._history_ptr] = {'event': event_name, 'data': data, 'ts': time.time()}
            self._history_ptr = (self._history_ptr + 1) % self._max_history
            subscribers = self._subscribers.get(event_name, [])
            if not subscribers:
                return
            subscribers = subscribers.copy()
        if synchronous or is_critical:
            for cb in subscribers:
                try:
                    cb(data)
                except Exception as e:
                    print(f'[BUS] Fault in FastPath: {e}')
        else:
            for cb in subscribers:
                self._executor.submit(lambda c=cb, e=event_name, d=data: self._safe_invoke(c, e, d))