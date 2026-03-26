# Generated method: SovereignScheduler.__init__
from dataclasses import dataclass, field
import time

class SovereignScheduler:
    def __init__(self, kernel):
        self.kernel = kernel
        self.tasks: list[Task] = []
        self.current_task_idx = -1
        self.last_switch_ts = time.time()