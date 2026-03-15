from sigma_core.interfaces.system_interfaces import ISchedulingStrategy
import random

from ._base import ChaosStrategy

class ChaosStrategy:
    def select_next(self, queue):
        if not queue:
            return None
        idx = random.randint(0, len(queue) - 1)
        return queue.pop(idx)