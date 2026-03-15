# Generated method: SigmaControlCenter.toggle_state
from typing import Dict, List, Any

class SigmaControlCenter:
    def toggle_state(self, key: str) -> str:
        """USP: Atomic state change for critical OS switches."""
        if key not in self._toggles:
            return 'Error: Unknown toggle in Sovereign Registry.'
        current = self._toggles[key]
        if isinstance(current, bool):
            self._toggles[key] = not current
        else:
            return f"Error: '{key}' is a multi-state parameter. Use set_value()."
        return f"ControlCenter: '{key}' is now {('ON' if self._toggles[key] else 'OFF')}."