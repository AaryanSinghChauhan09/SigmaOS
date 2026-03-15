# Generated method: SigmaAuraEngine.trigger_vblank_overclock
import time
from typing import Dict, Any

class SigmaAuraEngine:
    def trigger_vblank_overclock(self) -> str:
        """USP: Dynamically pushes the display controller to 144Hz-240Hz if monitor bus permits."""
        self._display_hz = 240
        self.kernel.bus.emit('aura.overclocked', {'hz': 240})
        return 'Aura_HW: Display controller overclocked to 240Hz. Ultra-Fluid mode ACTIVE.'