# Generated method: SigmaSovereignWatchdog._watchdog_loop
import os
import sys
import time
import threading
import platform
import subprocess
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignWatchdog:
    def _watchdog_loop(self):
        """Background loop — every 60s checks system health."""
        while self.is_running:
            time.sleep(60)
            self.auto_heal()