# Generated method: AutomationEngine.schedule_task
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    def schedule_task(self, name: str, interval_sec: int, task: Callable):
        self.scheduled_tasks.append({'name': name, 'interval': interval_sec, 'task': task, 'last_run': time.time()})