# Generated method: SovereignScheduler.health_check
from dataclasses import dataclass, field
import time

class SovereignScheduler:
    def health_check(self) -> str:
        return f'OK — Scheduler: {len(self.tasks)} tasks managed. Last switch {int((time.time() - self.last_switch_ts) * 1000)}ms ago.'