from sigma_core.interfaces.system_interfaces import ISchedulingStrategy
import random

from ._base import EnergyEfficientStrategy

class EnergyEfficientStrategy:
    def select_next(self, queue):
        if not queue:
            return None
        queue.sort(key=lambda x: x.get('complexity', 10))
        return queue.pop(0)