# Generated method: AdaptiveGovernor.start_service
from typing import Dict, Any, List

class AdaptiveGovernor:
    def start_service(self) -> str:
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.subscribe('mode.change', self._on_mode_change)
            self.kernel.bus.subscribe('eco.green_window', self._on_eco_window)
        return 'Adaptive Governor: Orchestration Mesh Online.'