from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import PowerManager

class PowerManager:
    def execute(self, action=None):
        return f'POWER_MODE_{self._mode}'