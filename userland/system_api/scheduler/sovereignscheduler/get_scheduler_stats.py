# Generated method: SovereignScheduler.get_scheduler_stats
from dataclasses import dataclass, field
import time

class SovereignScheduler:
    def get_scheduler_stats(self) -> dict:
        return {'Total_Tasks': len(self.tasks), 'Active_PID': self.tasks[self.current_task_idx].id if self.current_task_idx >= 0 else None, 'Mode': 'Round-Robin (Standard)', 'Quanta': '100ms'}