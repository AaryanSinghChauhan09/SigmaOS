# Generated method: MinimalistController.health_check
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class MinimalistController:
    def health_check(self) -> str:
        return f"OK — Mode: {self.active_mode} | Stealth: {('ACTIVE' if self.active_mode == 'MINIMAL' else 'STANDBY')}"