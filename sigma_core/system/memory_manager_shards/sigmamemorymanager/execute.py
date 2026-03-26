from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import SigmaMemoryManager

class SigmaMemoryManager:
    def execute(self, action, size=0):
        if action == 'ALLOCATE':
            if self._used + size <= self._total:
                self._used += size
                return 'SUCCESS'
            return 'OOM'
        elif action == 'DEALLOCATE':
            self._used = max(0, self._used - size)
            return 'SUCCESS'
        return 'UNKNOWN_ACTION'