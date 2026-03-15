# Generated method: SigmaStabilityWatchdog._watchdog_loop
import time
import threading
import collections
from typing import Dict, List, Any

class SigmaStabilityWatchdog:
    def _watchdog_loop(self):
        while not self._stop_event.is_set():
            total_fails = sum(self._failures.values())
            if total_fails > 10:
                self._trigger_survival_mode()
            time.sleep(2.0)