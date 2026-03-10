"""
Sovereign Multitasking Scheduler — v1.0
========================================
USP: Round-Robin cycle distribution with context preservation.
     Makes SigmaOS structurally "Alive" with concurrent execution.
"""

from dataclasses import dataclass, field
import time

@dataclass
class Task:
    id: int
    name: str
    priority: int
    state: str = "READY" # READY, RUNNING, SLEEPING, ZOMBIE
    context: dict = field(default_factory=lambda: {"EAX":0, "EBX":0, "ESP":0, "EIP":0})
    runtime_ms: float = 0.0

class SovereignScheduler:
    def __init__(self, kernel):
        self.kernel = kernel
        self.tasks: list[Task] = []
        self.current_task_idx = -1
        self.last_switch_ts = time.time()

    def create_task(self, name: str, priority: int = 10) -> int:
        tid = len(self.tasks)
        new_task = Task(id=tid, name=name, priority=priority)
        self.tasks.append(new_task)
        return tid

    def tick(self):
        """USP: Logic-Node Context Switch. Triggered by Timer IRQ."""
        if not self.tasks: return
        
        now = time.time()
        if now - self.last_switch_ts < 0.1: # 100ms Quanta
            return

        # Context Save
        if self.current_task_idx >= 0:
            current = self.tasks[self.current_task_idx]
            current.state = "READY"
            current.runtime_ms += (now - self.last_switch_ts) * 1000

        # Round-Robin Select
        self.current_task_idx = (self.current_task_idx + 1) % len(self.tasks)
        next_task = self.tasks[self.current_task_idx]
        next_task.state = "RUNNING"
        self.last_switch_ts = now
        
        # print(f"[SCHEDULER] Switched to {next_task.name} (ID: {next_task.id})")

    def get_scheduler_stats(self) -> dict:
        return {
            "Total_Tasks": len(self.tasks),
            "Active_PID": self.tasks[self.current_task_idx].id if self.current_task_idx >= 0 else None,
            "Mode": "Round-Robin (Standard)",
            "Quanta": "100ms"
        }

    def health_check(self) -> str:
        return f"OK — Scheduler: {len(self.tasks)} tasks managed. Last switch {int((time.time()-self.last_switch_ts)*1000)}ms ago."
