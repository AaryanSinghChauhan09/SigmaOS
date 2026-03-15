# Generated method: SigmaHardwareWarden.health_check
from typing import Dict, List, Any

class SigmaHardwareWarden:
    def health_check(self) -> str:
        return f'OK — {len(self._devices)} hardware devices wardenized.'