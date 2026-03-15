# Generated method: ResourceMonitor.__init__
import time
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class ResourceMonitor:
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.last_capture = time.time()
        self.metrics = {'load_average': 0.0, 'memory_pressure': 0.0, 'io_wait': 0.0}