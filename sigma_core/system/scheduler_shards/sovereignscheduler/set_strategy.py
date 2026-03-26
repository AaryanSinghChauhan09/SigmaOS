from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy

from ._base import SovereignScheduler

class SovereignScheduler:
    def set_strategy(self, strategy: ISchedulingStrategy):
        print(f'[SCHEDULER] Switching Strategy to: {strategy.__class__.__name__}')
        self._strategy = strategy