# Generated method: ProcessOrchestrator._worker_loop
import threading
import time
import queue
from typing import Dict, Any, List, Callable
import os
import sys
from sigma_core.event_bus import EventBus

class ProcessOrchestrator:
    def _worker_loop(self):
        while self._running:
            try:
                priority, mission_id, fn = self.task_queue.get(timeout=1)
                self.active_tasks[mission_id] = 'EXECUTING'
                result = fn()
                self.active_tasks[mission_id] = 'COMPLETED'
                if self.kernel:
                    self.kernel.bus.emit('mission.complete', {'id': mission_id, 'res': result})
            except queue.Empty:
                continue
            except Exception as e:
                print(f'[ORCHESTRATOR] Mission Failure ({mission_id}): {e}')
                self.active_tasks[mission_id] = 'FAILED'