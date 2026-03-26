from sigma_core.interfaces.system_interfaces import ISchedulingStrategy
import random

from ._base import PerformanceStrategy

class PerformanceStrategy:
    def select_next(self, queue):
        if not queue:
            return None
        queue.sort(key=lambda x: x.get('priority', 0), reverse=True)
        return queue.pop(0)