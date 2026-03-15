from sigma_core.interfaces.system_interfaces import ISchedulingStrategy
import random


class ChaosStrategy(ISchedulingStrategy):
    """Strategic randomness for stress testing (Chaos Monkey)."""