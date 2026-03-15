# Generated method: EventBus._safe_invoke
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def _safe_invoke(self, callback: Callable, event_name: str, data: Any) -> None:
        try:
            callback(data)
        except Exception as e:
            print(f'[BUS] Fault in WorkerPath ({event_name}): {e}')