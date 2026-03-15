# Generated method: SigmaSyncEngine.start_service
import time
import json
import threading
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncEngine:
    def start_service(self):
        if not self._running:
            self._running = True
            self._sync_thread = threading.Thread(target=self._clipboard_watcher, daemon=True)
            self._sync_thread.start()
            if self.kernel and hasattr(self.kernel, 'bus'):
                self.kernel.bus.subscribe('ghostchat.msg_received', self._on_handoff_received)
            self.log_event('sync_start', {'status': 'ACTIVE'})
        return 'Sync Engine: Handoff Sentinel Online.'