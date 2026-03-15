"""
Auto-split from sigma_core\system\automation_engine.py — AutomationEngine._automation_loop
"""

import time
import threading
from typing import Dict, Any, List, Callable, Optional



class AutomationEngine:
    def _automation_loop(self):
        while self._running:
            now = time.time()
            for task in self.scheduled_tasks:
                if now - task['last_run'] >= task['interval']:
                    try:
                        task['task']()
                    except Exception:
                        pass
                    task['last_run'] = now
            if self.kernel and hasattr(self.kernel, 'hal'):
                usage = self.kernel.hal.get_hardware_state()
                cpu_load = float(str(usage.get('cpu_load', '0%')).replace('%', ''))
                ram_load = float(str(usage.get('ram_load', '0%')).replace('%', ''))
                if ram_load > 90:
                    print(f'[AUTOMATION] High RAM Pressure detected ({ram_load}%). Triggering Boost.')
                    self.execute_workflow('performance.boost')
                if cpu_load > 95:
                    print(f'[AUTOMATION] Severe CPU Load detected ({cpu_load}%). Triggering Power Save.')
                    self.execute_workflow('power.save')
            time.sleep(5)
