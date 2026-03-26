from sigma_core.interfaces.system_interfaces import ISchedulingStrategy
import random


class PerformanceStrategy(ISchedulingStrategy):
    """Priority-driven high performance strategy."""