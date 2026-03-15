from ..interfaces.base_sovereign import SovereignModule
from ..interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from .scheduler_strategies import PerformanceStrategy

class SovereignScheduler(SovereignModule, IScheduler):
    """
    Sovereign Scheduler.
    Uses Composition over Inheritance for scheduling logic.
    """
    def __init__(self):
        super().__init__("SOVEREIGN_SCHEDULER")
        self._queue = []
        self._strategy = PerformanceStrategy() # Default strategy

    def set_strategy(self, strategy: ISchedulingStrategy):
        print(f"[SCHEDULER] Switching Strategy to: {strategy.__class__.__name__}")
        self._strategy = strategy

    def schedule_task(self, task_id, priority, complexity=5):
        self._queue.append({
            "id": task_id, 
            "priority": priority, 
            "complexity": complexity
        })
        print(f"[SCHEDULER] Task Queued: {task_id}")

    def execute(self, action=None):
        if action == "DISPATCH_NEXT":
            task = self._strategy.select_next(self._queue)
            if task:
                print(f"[SCHEDULER] Dispatching Task: {task['id']} using {self._strategy.__class__.__name__}")
                return task
            return "IDLE"
        return f"SCHEDULER_QUEUE_SIZE_{len(self._queue)}"

    def initialize(self):
        print("[SCHEDULER] Sovereign Task Master Online.")

    def shutdown(self):
        self._queue.clear()
        print("[SCHEDULER] Task Master Shutdown.")

    def health_check(self) -> bool:
        return True

def get_scheduler() -> SovereignScheduler:
    return SovereignScheduler()
