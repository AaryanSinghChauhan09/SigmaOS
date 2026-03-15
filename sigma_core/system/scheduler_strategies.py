from ..interfaces.system_interfaces import ISchedulingStrategy
import random

class PerformanceStrategy(ISchedulingStrategy):
    """Priority-driven high performance strategy."""
    def select_next(self, queue):
        if not queue: return None
        # Sort by priority (higher is better)
        queue.sort(key=lambda x: x.get('priority', 0), reverse=True)
        return queue.pop(0)

class EnergyEfficientStrategy(ISchedulingStrategy):
    """Strategy that prioritizes low-complexity tasks to save power."""
    def select_next(self, queue):
        if not queue: return None
        # Sort by complexity (lower is better for energy)
        queue.sort(key=lambda x: x.get('complexity', 10))
        return queue.pop(0)

class ChaosStrategy(ISchedulingStrategy):
    """Strategic randomness for stress testing (Chaos Monkey)."""
    def select_next(self, queue):
        if not queue: return None
        idx = random.randint(0, len(queue) - 1)
        return queue.pop(idx)
