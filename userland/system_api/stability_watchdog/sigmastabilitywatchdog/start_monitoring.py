# Generated method: SigmaStabilityWatchdog.start_monitoring
import time
import threading
import collections
from typing import Dict, List, Any

class SigmaStabilityWatchdog:
    def start_monitoring(self):
        self._stop_event.clear()
        self._monitor_thread = threading.Thread(target=self._watchdog_loop, daemon=True)
        self._monitor_thread.start()