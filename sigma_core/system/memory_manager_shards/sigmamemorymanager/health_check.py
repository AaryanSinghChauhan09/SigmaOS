from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import SigmaMemoryManager

class SigmaMemoryManager:
    def health_check(self):
        return True