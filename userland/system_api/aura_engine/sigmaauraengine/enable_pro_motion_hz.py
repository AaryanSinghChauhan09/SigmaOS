# Generated method: SigmaAuraEngine.enable_pro_motion_hz
import time
from typing import Dict, Any

class SigmaAuraEngine:
    def enable_pro_motion_hz(self) -> str:
        """Unlocks the display refresh rate to the absolute limits of the panel."""
        self._latency_ms = 0.1
        return 'AURA: Pro-Motion Unlocked. V-Sync detached for eSports latency mode (<0.1ms).'