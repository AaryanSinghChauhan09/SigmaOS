# Generated method: ForensicSentinel.stop_service
import time
import threading
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class ForensicSentinel:
    def stop_service(self):
        self._sentinel_running = False