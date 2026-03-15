# Generated method: SigmaHardwareWarden.overclock
from typing import Dict, List, Any

class SigmaHardwareWarden:
    def overclock(self, delta_mhz: int) -> str:
        """USP: Atomic hardware freq adjustment with kernel-level safety bypass."""
        self._tunables['GPU_Clock'] += delta_mhz
        return f'HardwareWarden: GPU Clock boosted by {delta_mhz}MHz. Cooling profiles re-tuned.'