# Generated method: SigmaSyncEngine._clipboard_watcher
import time
import json
import threading
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncEngine:
    def _clipboard_watcher(self):
        """Monitors local clipboard and broadcasts changes to peers."""
        while self._running:
            if self.sync_active and self.last_clipboard:
                if self.kernel and hasattr(self.kernel, 'ghostchat'):
                    payload = {'type': 'HANDOFF_CLIPBOARD', 'content': self.last_clipboard, 'ts': time.time()}
                    self.kernel.ghostchat.send_message(json.dumps(payload))
                    self.stats['handoffs_completed'] += 1
            time.sleep(2)