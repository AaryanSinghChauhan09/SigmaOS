# Generated method: SigmaOmniStudio.health_check
from typing import Dict
import time

class SigmaOmniStudio:
    def health_check(self) -> str:
        return f"OK — Omni-Studio Engine Active. Current Mode: {self.active_mode or 'Idle'}."