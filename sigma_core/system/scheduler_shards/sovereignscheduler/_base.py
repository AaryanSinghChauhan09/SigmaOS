from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.system_interfaces import IScheduler, ISchedulingStrategy
from sigma_core.system.scheduler_strategies import PerformanceStrategy


class SovereignScheduler(SovereignModule, IScheduler):
    __slots__ = ('_queue', '_strategy')
    '\n    Sovereign Scheduler.\n    Uses Composition over Inheritance for scheduling logic.\n    '