# Generated method: SigmaSovereignTaskScheduler.schedule_mission
import os
import sys
import time
import threading
import queue
from typing import Dict, List, Any, Optional, Callable
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignTaskScheduler:
    def schedule_mission(self, name: str, mission_fn: Callable, priority: int=1) -> str:
        """Schedules a mission with a specific priority."""
        self.task_queue.put((priority, name, mission_fn))
        return f"Mission '{name}' queued with priority {priority}."