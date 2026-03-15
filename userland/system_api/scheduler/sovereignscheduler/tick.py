# Generated method: SovereignScheduler.tick
from dataclasses import dataclass, field
import time

class SovereignScheduler:
    def tick(self):
        """USP: Logic-Node Context Switch. Triggered by Timer IRQ."""
        if not self.tasks:
            return
        now = time.time()
        if now - self.last_switch_ts < 0.1:
            return
        if self.current_task_idx >= 0:
            current = self.tasks[self.current_task_idx]
            current.state = 'READY'
            current.runtime_ms += (now - self.last_switch_ts) * 1000
        self.current_task_idx = (self.current_task_idx + 1) % len(self.tasks)
        next_task = self.tasks[self.current_task_idx]
        next_task.state = 'RUNNING'
        self.last_switch_ts = now