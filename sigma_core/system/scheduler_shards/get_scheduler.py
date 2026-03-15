from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy
from ..sovereignscheduler._base import SovereignScheduler

def get_scheduler() -> SovereignScheduler:
    return SovereignScheduler()