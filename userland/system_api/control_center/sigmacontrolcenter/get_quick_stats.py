# Generated method: SigmaControlCenter.get_quick_stats
from typing import Dict, List, Any

class SigmaControlCenter:
    def get_quick_stats(self) -> Dict:
        """Returns condensed OS status for the fly-out UI."""
        return {'Toggles': self._toggles, 'Volume': '85%', 'Brightness': '70%', 'Network': 'Offline-Guard Active', 'Performance_Mode': self.kernel.modes.get_active_profile()['Mode']}