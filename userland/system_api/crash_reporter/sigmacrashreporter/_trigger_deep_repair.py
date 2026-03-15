# Generated method: SigmaCrashReporter._trigger_deep_repair
import time
import uuid
import threading
import json
from typing import Dict, List, Any

class SigmaCrashReporter:
    def _trigger_deep_repair(self, module: str):
        """USP: Escalates to SelfRepairEngine for a full module re-build."""
        if self.kernel.repair_engine:
            self.kernel.bus.emit('crash.deep_repair_triggered', {'module': module})
            self.kernel.repair_engine.repair_module(module, severity_z=5.0)