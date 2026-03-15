# Generated method: SigmaSovereignTaskScheduler._scheduler_loop
import os
import sys
import time
import threading
import queue
from typing import Dict, List, Any, Optional, Callable
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignTaskScheduler:
    def _scheduler_loop(self):
        while self.is_running:
            try:
                if not self.task_queue.empty():
                    priority, name, mission_fn = self.task_queue.get()
                    self._balance_resources(priority)
                    mission_fn()
                    self.stats['tasks_completed'] += 1
                else:
                    time.sleep(1)
            except Exception as e:
                print(f'[SCHEDULER_ERR] {e}')