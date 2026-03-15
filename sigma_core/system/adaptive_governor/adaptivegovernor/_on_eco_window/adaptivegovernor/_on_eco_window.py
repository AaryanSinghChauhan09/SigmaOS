# Generated method: AdaptiveGovernor._on_eco_window
from typing import Dict, Any, List

class AdaptiveGovernor:
    def _on_eco_window(self, payload: Dict[str, Any]):
        """Responds to high carbon intensity by throttling non-critical shards."""
        active = payload.get('active', False)
        if active:
            self.state['eco_priority'] = True
            if self.kernel.perf:
                self.kernel.perf.apply_tuning('Eco')
        else:
            self.state['eco_priority'] = False