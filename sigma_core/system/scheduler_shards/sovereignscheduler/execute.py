from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy

from ._base import SovereignScheduler

class SovereignScheduler:
    def execute(self, action=None):
        if action == 'DISPATCH_NEXT':
            task = self._strategy.select_next(self._queue)
            if task:
                print(f"[SCHEDULER] Dispatching Task: {task['id']} using {self._strategy.__class__.__name__}")
                return task
            return 'IDLE'
        return f'SCHEDULER_QUEUE_SIZE_{len(self._queue)}'