from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy

from ._base import SovereignScheduler

class SovereignScheduler:
    def __init__(self):
        super().__init__('SOVEREIGN_SCHEDULER')
        self._queue = []
        self._strategy = PerformanceStrategy()