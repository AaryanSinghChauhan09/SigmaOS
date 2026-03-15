# Generated method: OmniSentinel.__init__
import time
import threading
from typing import Dict, Any

class OmniSentinel:
    def __init__(self, stats: dict, kernel=None, launch_preset_fn=None):
        self.stats = stats
        self.kernel = kernel
        self.launch_preset_fn = launch_preset_fn
        self._running = False
        self._thread: threading.Thread | None = None