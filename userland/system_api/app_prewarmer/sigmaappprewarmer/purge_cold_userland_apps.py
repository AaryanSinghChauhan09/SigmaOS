"""
Auto-split from userland\system_api\app_prewarmer.py — SigmaAppPrewarmer.purge_cold_userland_apps
"""

import time
import uuid
import threading
from typing import Dict, List, Any, Optional



class SigmaAppPrewarmer:
    def purge_cold_userland_apps(self) -> str:
        """Frees all shadow memory instantly. Usually called by ModeManager on mode switch."""
        with self._lock:
            freed = 0.0
            for shadow in self._shadow_pool.values():
                freed += shadow.memory_reserved_mb
                if self.kernel.memory:
                    self.kernel.memory.free('shadow', shadow.memory_reserved_mb)
            count = len(self._shadow_pool)
            self._shadow_pool.clear()
        return f'Prewarmer: Hot-RAM cleared. Evicted {count} shadows, freed {freed}MB.'
