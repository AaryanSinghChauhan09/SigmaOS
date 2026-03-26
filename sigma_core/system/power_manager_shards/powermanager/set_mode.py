from sigma_core.interfaces.base_sovereign import SovereignModule

from ._base import PowerManager

class PowerManager:
    def set_mode(self, mode):
        print(f'[POWER] Mode switched to: {mode}')
        self._mode = mode