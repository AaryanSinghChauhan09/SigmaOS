# Generated method: SigmaProjects.get_active_task_info
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def get_active_task_info(self) -> Dict:
        if self._active_timer_task:
            return {'tid': self._active_timer_task, 'elapsed': time.time() - self._active_timer_start, 'title': self._tasks[self._active_timer_task].title if self._active_timer_task != 'GLOBAL' else 'Global Productivity'}
        return None