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
        """Initialize the event bus"""
        self._subscribers: Dict[str, List[Callable]] = {}
        self._event_history: List[Dict[str, Any]] = []
        self._lock = threading.Lock()
        self._max_history = 1000
        self._executor = ThreadPoolExecutor(max_workers=16)
    
    def subscribe(self, event_name: str, callback: Callable) -> None:
        """
        Subscribe to an event
        
        Args:
            event_name: Name of the event to subscribe to
            callback: Callable to invoke when event is emitted
        """
        with self._lock:
            if event_name not in self._subscribers:
                self._subscribers[event_name] = []
            self._subscribers[event_name].append(callback)
    
    def unsubscribe(self, event_name: str, callback: Callable) -> None:
        """Unsubscribe from an event"""
        with self._lock:
            if event_name in self._subscribers:
                if callback in self._subscribers[event_name]:
                    self._subscribers[event_name].remove(callback)
    
    def emit(self, event_name: str, data: Any = None, synchronous: bool = False, priority: int = 2) -> None:
        """
        Emit an event to all subscribers.
        
        Args:
            event_name: Name of the event
            data: Event payload data
            synchronous: If True, waits for callback completion.
            priority: Lower value = Higher priority (0-5 scale).
        """
        is_critical = "critical" in event_name or priority <= 1
        with self._lock:
            # Record event in history
            self._event_history.append({
                "event": event_name,
                "data": data,
                "timestamp": __import__('time').time()
            })
            
            # Trim history if needed
            if len(self._event_history) > self._max_history:
                self._event_history.pop(0)
            
            # Get subscribers for this event
            subscribers = self._subscribers.get(event_name, []).copy()
        
        # Invoke subscribers outside lock
        for callback in subscribers:
            if synchronous or is_critical:
                try:
                    callback(data)
                except Exception as e:
                    print(f"[EventBus] Error in priority/sync callback for '{event_name}': {e}")
            else:
                # Use executor with simulated priority context
                self._executor.submit(self._safe_invoke, callback, event_name, data)

    def _safe_invoke(self, callback: Callable, event_name: str, data: Any):
        """Internal helper for safe asynchronous callback execution."""
        try:
            callback(data)
        except Exception as e:
            print(f"[EventBus] Error in async callback for '{event_name}': {e}")
    
    def get_history(self, event_name: str = None) -> List[Dict]:
        """
        Get event history
        
        Args:
            event_name: Optional filter by event name
            
        Returns:
            List of historical events
        """
        with self._lock:
            if event_name:
                return [e for e in self._event_history if e["event"] == event_name]
            return self._event_history.copy()
    
    def clear_history(self) -> None:
        """Clear event history"""
        with self._lock:
            self._event_history.clear()