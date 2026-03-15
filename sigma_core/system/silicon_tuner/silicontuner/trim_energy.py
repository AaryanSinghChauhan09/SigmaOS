# Generated method: SiliconTuner.trim_energy
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SiliconTuner:
    def trim_energy(self):
        """Forces immediate energy-saving re-tuning."""
        hal = getattr(self.kernel, 'hal', None)
        if hal:
            hal.set_process_priority('Low')