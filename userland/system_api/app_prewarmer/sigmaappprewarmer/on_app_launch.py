"""
Auto-split from userland\system_api\app_prewarmer.py — SigmaAppPrewarmer.on_app_launch
"""

import time
import uuid
import threading
from typing import Dict, List, Any, Optional



class SigmaAppPrewarmer:
    def on_app_launch(self, app_name: str) -> str:
        """Intercepts actual app launch. If warmed, unpauses instantly."""
        with self._lock:
            if self.last_launched_app:
                self._reinforce_prediction(self.last_launched_app, app_name)
            self.last_launched_app = app_name
            if app_name in self._shadow_pool:
                self._cache_hits += 1
                shadow = self._shadow_pool.pop(app_name)
                if self.kernel and hasattr(self.kernel, 'memory') and self.kernel.memory:
                    self.kernel.memory.free('shadow', shadow.memory_reserved_mb)
                if shadow.hardware_locked and self.kernel and hasattr(self.kernel, 'hal'):
                    self.kernel.hal.unlock_memory('shadow_pages', int(shadow.memory_reserved_mb * 1024 * 1024))
                self._predict_and_warm(app_name)
                return f"INSTANT LAUNCH: '{app_name}' unpaused from Shadow RAM (0.0ms delay)."
            else:
                self._cache_misses += 1
                self._predict_and_warm(app_name)
                return f"COLD LAUNCH: '{app_name}' booted from disk."
