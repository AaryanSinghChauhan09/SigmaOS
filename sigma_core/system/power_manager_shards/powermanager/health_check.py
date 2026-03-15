from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import PowerManager

class PowerManager:
    def health_check(self):
        return True