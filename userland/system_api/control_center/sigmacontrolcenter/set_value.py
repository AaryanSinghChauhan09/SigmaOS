# Generated method: SigmaControlCenter.set_value
from typing import Dict, List, Any

class SigmaControlCenter:
    def set_value(self, key: str, value: Any) -> str:
        """USP: Set hierarchical values for OS performance tiers."""
        if key not in self._toggles:
            return 'Error: Key not found.'
        self._toggles[key] = value
        return f"ControlCenter: Set '{key}' to '{value}'."