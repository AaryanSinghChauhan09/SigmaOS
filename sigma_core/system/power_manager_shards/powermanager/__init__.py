from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import PowerManager

class PowerManager:
    def __init__(self):
        super().__init__('POWER_MANAGER')
        self._mode = 'HIGH_PERFORMANCE'