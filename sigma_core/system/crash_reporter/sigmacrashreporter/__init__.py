# Generated method: SigmaCrashReporter.__init__
import time
import uuid
import threading
import json
from typing import Dict, List, Any

class SigmaCrashReporter:
    def __init__(self, kernel):
        self.kernel = kernel
        self._reports = []
        self._lock = threading.Lock()
        self._recurrent_threshold = 3
        self._module_crash_map: Dict[str, int] = {}