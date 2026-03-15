"""
Auto-split from userland\system_api\app_prewarmer.py — SigmaAppPrewarmer.prewarm
"""

import time
import uuid
import threading
from typing import Dict, List, Any, Optional



class SigmaAppPrewarmer:
    def prewarm(self, app_name: str, priority: str='normal') -> bool:
        """Spawns a dormant process in memory to guarantee 0ms launch."""
        with self._lock:
            if len(self._shadow_pool) >= 5:
                oldest = min(self._shadow_pool.values(), key=lambda p: p.warmed_at)
                self._shadow_pool.pop(oldest.app_name, None)
                if self.kernel and hasattr(self.kernel, 'memory') and self.kernel.memory:
                    self.kernel.memory.free('shadow', oldest.memory_reserved_mb)
            if app_name not in self._shadow_pool:
                shadow = ShadowProcess(app_name)
                self._shadow_pool[app_name] = shadow
                if self.kernel and hasattr(self.kernel, 'memory') and self.kernel.memory:
                    self.kernel.memory.allocate('shadow', shadow.memory_reserved_mb, 'Prewarmer')
                if self.kernel and hasattr(self.kernel, 'hal'):
                    self.kernel.hal.lock_memory('shadow_pages', int(shadow.memory_reserved_mb * 1024 * 1024))
                    shadow.hardware_locked = True
                if self.kernel and hasattr(self.kernel, 'bus'):
                    self.kernel.bus.emit('pre_warm.spawned', {'app': app_name, 'pid': shadow.pid})
                return True
        return False
