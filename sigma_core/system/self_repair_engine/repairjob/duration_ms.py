# Generated method: RepairJob.duration_ms
import time
import threading
import random
from typing import Dict, List, Any, Optional
from .interfaces import SigmaModuleBase
from sigma_core.system.interfaces import SigmaModuleBase

class RepairJob:
    @property
    def duration_ms(self) -> float:
        if self.end_ts:
            return (self.end_ts - self.start_ts) * 1000
        return 0.0