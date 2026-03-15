from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import SigmaMemoryManager

class SigmaMemoryManager:
    def __init__(self, capacity_mb=4096):
        super().__init__('MEMORY_MANAGER')
        self._total = capacity_mb
        self._used = 0
        self._blocks = []