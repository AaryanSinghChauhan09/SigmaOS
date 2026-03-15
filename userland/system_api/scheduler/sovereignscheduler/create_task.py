# Generated method: SovereignScheduler.create_task
from dataclasses import dataclass, field
import time

class SovereignScheduler:
    def create_task(self, name: str, priority: int=10) -> int:
        tid = len(self.tasks)
        new_task = Task(id=tid, name=name, priority=priority)
        self.tasks.append(new_task)
        return tid