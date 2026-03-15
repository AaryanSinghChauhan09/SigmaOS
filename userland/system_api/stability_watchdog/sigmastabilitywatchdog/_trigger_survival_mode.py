# Generated method: SigmaStabilityWatchdog._trigger_survival_mode
import time
import threading
import collections
from typing import Dict, List, Any

class SigmaStabilityWatchdog:
    def _trigger_survival_mode(self):
        """USP: Shuts down non-essential UI and networking to preserve the Kernel."""
        self.kernel.bus.emit('watchdog.survival_mode', {'active': True})
        if self.kernel.mode_manager:
            self.kernel.mode_manager.switch_mode('Stability')