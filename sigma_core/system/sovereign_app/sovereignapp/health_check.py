# Generated method: SovereignApp.health_check
from sigma_core.system.interfaces import SigmaModuleBase

class SovereignApp:
    def health_check(self) -> str:
        return f'OK - {self.app_id} ACTIVE'