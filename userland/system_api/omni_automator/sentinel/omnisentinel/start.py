# Generated method: OmniSentinel.start
import time
import threading
from typing import Dict, Any

class OmniSentinel:
    def start(self):
        """Start the proactive sentinel daemon thread."""
        if not self._running:
            self._running = True
            self._thread = threading.Thread(target=self._cycle, daemon=True)
            self._thread.start()
            print('[OMNI] Proactive Sentinel [ONLINE].')