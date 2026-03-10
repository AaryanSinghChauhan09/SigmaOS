"""
SigmaOS Shadow-State Recovery Engine (Apex v1.0)
================================================
USP: Near-Zero Downtime (Zero-Lag Recovery).
     Maintains a 'Hot-Standby' copy of critical module states.
"""

import time
import copy
import threading
from typing import Dict, Any

class SigmaShadowState:
    def __init__(self, kernel):
        self.kernel = kernel
        self._shadows: Dict[str, Any] = {}
        self._last_sync: Dict[str, float] = {}
        self._is_recovering = False

    def capture_shadow(self, module_key: str):
        """Creates a serialized snapshot of a module's essential state."""
        module = self.kernel.registry.get(module_key)
        if not module:
            return

        # Intelligent State Extraction (Safe Filtering)
        state = {}
        if hasattr(module, "get_state") and callable(module.get_state):
            state = module.get_state()
        elif hasattr(module, "__dict__"):
            # Filter out private (__double) attributes and non-pickleable types
            blacklist_types = ("Lock", "RLock", "Event", "Thread", "ModuleRegistry", "EventBus", "SigmaKernel")
            for k, v in module.__dict__.items():
                if k.startswith('__') or callable(v):
                    continue
                type_name = str(type(v))
                if any(bt in type_name for bt in blacklist_types):
                    continue
                state[k] = v
        
        try:
            self._shadows[module_key] = copy.deepcopy(state)
            self._last_sync[module_key] = time.time()
            self.kernel.bus.emit("shadow.captured", {"module": module_key, "ts": self._last_sync[module_key]})
        except Exception as e:
            # If deepcopy still fails, we log it to the event bus
            self.kernel.bus.emit("shadow.capture_error", {"module": module_key, "error": str(e)})

    def hot_swap(self, module_key: str) -> bool:
        """
        USP: The 'Shadow Swap'. 
        Restores a module to its last known good state if the Watchdog trips it.
        """
        if module_key not in self._shadows:
            return False

        self._is_recovering = True
        module = self.kernel.registry.get(module_key)
        if not module:
            return False

        state = self._shadows[module_key]
        
        try:
            if hasattr(module, "set_state"):
                module.set_state(state)
            else:
                for k, v in state.items():
                    setattr(module, k, copy.deepcopy(v))
            
            self.kernel.bus.emit("shadow.swapped", {
                "module": module_key, 
                "msg": "Hot-Standby state RESTORED. System stabilized."
            })
            self._is_recovering = False
            return True
        except Exception as e:
            self.kernel.bus.emit("shadow.swap_failed", {"module": module_key, "error": str(e)})
            self._is_recovering = False
            return False

    def start_periodic_sync(self, interval=300):
        """Background sync of critical module shadows."""
        def _loop():
            while True:
                time.sleep(interval)
                critical_mods = ["update_manager", "energy_hub", "mesh_compute", "cog_fabric"]
                for mod in critical_mods:
                    self.capture_shadow(mod)
        
        t = threading.Thread(target=_loop, daemon=True)
        t.start()
        self.kernel.bus.emit("shadow.sync_started", {"interval": interval})

    def health_check(self) -> str:
        return f"OK — Shadow Recovery Active. Cached Modules: {list(self._shadows.keys())}"
