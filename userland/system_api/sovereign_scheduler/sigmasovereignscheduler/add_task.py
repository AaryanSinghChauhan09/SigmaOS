# Generated method: SigmaSovereignScheduler.add_task
import time
import json
import os
from datetime import datetime, timedelta
from typing import List, Dict, Any, Optional

class SigmaSovereignScheduler:
    def add_task(self, name: str, duration_min: int, priority: str='Medium', deadline: str=None):
        """USP: AI Auto-Scheduling. Finds the best gap in the calendar."""
        task = {'id': f'TASK-{int(time.time())}', 'name': name, 'duration': duration_min, 'priority': priority, 'deadline': deadline, 'status': 'QUEUED'}
        self.tasks.append(task)
        self._recompute_schedule()
        self.stats['tasks_auto_scheduled'] += 1
        return f"Scheduler: '{name}' auto-slotted into the next available high-priority gap."