# Generated method: RepairJob.complete
import time
import threading
import random
from typing import Dict, List, Any, Optional
from .interfaces import SigmaModuleBase
from sigma_core.system.interfaces import SigmaModuleBase

class RepairJob:
    def complete(self, success: bool, notes: str=''):
        self.end_ts = time.monotonic()
        self.success = success
        self.notes = notes