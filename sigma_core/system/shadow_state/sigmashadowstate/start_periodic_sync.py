# Generated method: SigmaShadowState.start_periodic_sync
import time
import copy
import threading
from typing import Dict, Any

class SigmaShadowState:
    def start_periodic_sync(self, interval=300):
        """Background sync of critical module shadows."""

        def _loop():
            while True:
                time.sleep(interval)
                critical_mods = ['update_manager', 'energy_hub', 'mesh_compute', 'cog_fabric']
                for mod in critical_mods:
                    self.capture_shadow(mod)
        t = threading.Thread(target=_loop, daemon=True)
        t.start()
        self.kernel.bus.emit('shadow.sync_started', {'interval': interval})