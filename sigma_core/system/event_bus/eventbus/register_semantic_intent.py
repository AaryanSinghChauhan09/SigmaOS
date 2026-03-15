# Generated method: EventBus.register_semantic_intent
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    def register_semantic_intent(self, intent: str, event_key: str):
        """CS/AI Principle: Semantic Routing. Maps natural language intent to a rigid event key."""
        with self._lock:
            if intent not in self._semantic_map:
                self._semantic_map[intent] = []
            self._semantic_map[intent].append(event_key)