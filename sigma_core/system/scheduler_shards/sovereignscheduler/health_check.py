from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy

from ._base import SovereignScheduler

class SovereignScheduler:
    def health_check(self) -> bool:
        return True