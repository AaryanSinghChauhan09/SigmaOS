from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy

from ._base import SovereignScheduler

class SovereignScheduler:
    def schedule_task(self, task_id, priority, complexity=5):
        self._queue.append({'id': task_id, 'priority': priority, 'complexity': complexity})
        print(f'[SCHEDULER] Task Queued: {task_id}')