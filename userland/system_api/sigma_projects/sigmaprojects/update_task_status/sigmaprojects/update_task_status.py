# Generated method: SigmaProjects.update_task_status
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def update_task_status(self, tid, status: TaskStatus):
        if tid in self._tasks:
            prev = self._tasks[tid].status
            self._tasks[tid].status = status
            if status == TaskStatus.DONE and prev != TaskStatus.DONE:
                if self.kernel and hasattr(self.kernel, 'routine_manager'):
                    self.kernel.routine_manager.process_trigger('task.done')
                elif self.kernel and hasattr(self.kernel, 'registry') and self.kernel.registry.get('routines'):
                    self.kernel.registry.get('routines').process_trigger('task.done')
            return True
        return False