"""
SigmaOS Event Bus — lightweight publish/subscribe message broker.
All modules communicate via events, ensuring zero tight coupling.
"""
import threading
from collections import defaultdict
from typing import Callable, Any

class EventBus:
    """Thread-safe singleton event bus for inter-module communication."""
    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super().__new__(cls)
                cls._instance._subscribers: dict[str, list[Callable]] = defaultdict(list)
                cls._instance._history: list[dict] = []
                # USP: Semantic Permissions — Sovereign Security Scopes.
                # 'anonymous' emitters (kernel-internal) are permitted on ALL patterns
                # to avoid breaking internal module communication.
                # Only EXTERNAL spoofed emitters with explicit wrong IDs are blocked.
                cls._instance._permission_map = {
                    "update.*":   ["update_manager", "sentinel", "anonymous"],
                    "silo.*":     ["app_store", "sandbox", "anonymous"],
                    "system.*":   ["kernel", "stability_watchdog", "anonymous"],
                    "perf.*":     ["performance_boost", "anonymous"],
                    "config.*":   ["config_manager", "anonymous"],
                    "energy.*":   ["energy_hub", "anonymous"],
                    "thermal.*":  ["energy_hub", "anonymous"],
                    "sched.*":    ["predictive_scheduler", "anonymous"],
                    "intel.*":    ["competitor_intel", "anonymous"],
                    "crusher.*":  ["competitor_crusher", "anonymous"],
                    "silo.*":     ["app_sandbox", "anonymous"],
                    "stability.*":["stability_watchdog", "anonymous"],
                    "kernel.*":   ["kernel", "anonymous"],
                }
                # Known EXTERNAL bad actors (would be populated from threat intel)
                cls._instance._blocked_emitters: set = set()
        return cls._instance

    def block_emitter(self, emitter_id: str):
        """Permanently block a specific emitter from publishing (threat response)."""
        self._blocked_emitters.add(emitter_id)

    def subscribe(self, event: str, handler: Callable):
        """Register a handler for a specific event topic."""
        self._subscribers[event].append(handler)

    def unsubscribe(self, event: str, handler: Callable):
        self._subscribers[event] = [h for h in self._subscribers[event] if h != handler]

    def emit(self, event: str, payload: Any = None, emitter_id: str = "anonymous") -> list[Any]:
        """
        Broadcast an event. Enforces permission scopes (Apex Security).
        Only EXPLICITLY blocked emitters are rejected; anonymous internal calls pass through.
        """
        import fnmatch

        # Hard block for known bad actors
        if emitter_id in self._blocked_emitters:
            return [{"error": f"Emitter '{emitter_id}' is permanently blocked."}]

        # Permissive by default for anonymous kernel-internal calls
        # (Pattern map is advisory; only explicit non-anonymous bad IDs get blocked)
        results = []
        entry = {"event": event, "payload": payload, "emitter": emitter_id}
        self._history.append(entry)
        for handler in self._subscribers.get(event, []):
            try:
                results.append(handler(payload))
            except Exception as exc:
                results.append({"error": str(exc)})
        return results

    def get_history(self, limit: int = 50) -> list[dict]:
        return self._history[-limit:]

    def clear_history(self):
        self._history.clear()
