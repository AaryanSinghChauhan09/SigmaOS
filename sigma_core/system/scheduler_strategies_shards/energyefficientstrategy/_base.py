from sigma_core.interfaces.system_interfaces import ISchedulingStrategy
import random


class EnergyEfficientStrategy(ISchedulingStrategy):
    """Strategy that prioritizes low-complexity tasks to save power."""