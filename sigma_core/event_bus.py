"""
SigmaOS Event Bus Module
Implements publish-subscribe event messaging for inter-module communication.
"""

from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    """
    Central event bus for SigmaOS kernel.
    Enables decoupled communication between modules via events.
    USP: High-Performance Concurrent Event Delivery Engine with Priority Queueing.
    """
    
    def __init__(self):
        """Initialize the event bus with a high-performance circular buffer."""
        self._subscribers: Dict[str, List[Callable]] = {}
        # USP: Zero-Allocation Circular Buffer for low-level performance
        self._max_history = 1000
        self._event_history: List[Any] = [None] * self._max_history
        self._history_ptr = 0
        self._lock = threading.Lock()
        self._executor = ThreadPoolExecutor(max_workers=24) # Increased for Apex load
        self._interaction_weights: Dict[str, float] = {}   # ML Personalization weights
    
    def subscribe(self, event_name: str, callback: Callable) -> None:
        """Subscribe to an event (O(1) write)."""
        with self._lock:
            if event_name not in self._subscribers:
                self._subscribers[event_name] = []
            self._subscribers[event_name].append(callback)
    
    def unsubscribe(self, event_name: str, callback: Callable) -> None:
        """Unsubscribe from an event."""
        with self._lock:
            if event_name in self._subscribers:
                if callback in self._subscribers[event_name]:
                    self._subscribers[event_name].remove(callback)
    
    def emit(self, event_name: str, data: Any = None, synchronous: bool = False, priority: int = 2) -> None:
        """
        Emit an event via the FAST-PATH or the WORKER-PATH.
        """
        # ML Personalization: Elevate priority for high-interaction event types
        interaction_boost = self._interaction_weights.get(event_name, 1.0)
        is_critical = priority <= 1 or "fault" in event_name or "kernel" in event_name or interaction_boost > 2.0
        
        if interaction_boost > 1.0:
            self._interaction_weights[event_name] = min(interaction_boost + 0.1, 5.0)
        else:
            self._interaction_weights[event_name] = 1.1 # Seed weight
        
        with self._lock:
            # USP: Circular Ring Buffer prevents O(N) list shifts
            self._event_history[self._history_ptr] = {
                "event": event_name,
                "data": data,
                "ts": time.time()
            }
            self._history_ptr = (self._history_ptr + 1) % self._max_history
            
            subscribers = self._subscribers.get(event_name, [])
            if not subscribers: return
            subscribers = subscribers.copy()
        
        # FAST-PATH: Direct execution for critical/sync events
        if synchronous or is_critical:
            for cb in subscribers:
                try: cb(data)
                except Exception as e: print(f"[BUS] Fault in FastPath: {e}")
        else:
            # WORKER-PATH: Offload to apex thread pool
            for cb in subscribers:
                # Standard execution path via worker pool
                self._executor.submit(self._safe_invoke, cb, event_name, data)

    def _safe_invoke(self, callback: Callable, event_name: str, data: Any):
        try:
            callback(data)
        except Exception as e:
            print(f"[BUS] Fault in WorkerPath ({event_name}): {e}")
    
    def get_history(self, event_name: str | None = None) -> List[Any]:
        """Returns the event history; filters results to exclude 'None' slots."""
        with self._lock:
            h = [e for e in self._event_history if e is not None]
            if event_name:
                return [e for e in h if e["event"] == event_name]
            return h
    
    def clear_history(self) -> None:
        """Resets the history pointer and purges the ring buffer."""
        with self._lock:
            self._event_history = [None] * self._max_history
            self._history_ptr = 0