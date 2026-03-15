# Generated method: AutomationEngine._check_thermal_envelope
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    def _check_thermal_envelope(self):
        if hasattr(self.kernel, 'energy'):
            metrics = self.kernel.energy.get_realtime_metrics()
            if metrics.get('thermal_status') == 'CRITICAL':
                self.execute_workflow('power.save')