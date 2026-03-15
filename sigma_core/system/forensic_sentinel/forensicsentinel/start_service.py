# Generated method: ForensicSentinel.start_service
import time
import threading
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class ForensicSentinel:
    def start_service(self) -> str:
        if not self._sentinel_running:
            self._sentinel_running = True
            t = threading.Thread(target=self._sentinel_loop, daemon=True)
            t.start()
            return 'Forensic-Sentinel: Proactive Healing ACTIVE.'
        return 'Forensic-Sentinel: Already running.'