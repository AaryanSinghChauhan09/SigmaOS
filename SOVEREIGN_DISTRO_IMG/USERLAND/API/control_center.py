"""
SigmaControlCenter: Universal Toggle & Control Hub.
==================================================
USP: One-click toggles for all OS performance and state variables.
Inspiration: iOS Control Center, macOS Menu Bar, Android Quick Toggles.
"""

from typing import Dict, List, Any

class SigmaControlCenter:
    def __init__(self, kernel):
        self.kernel = kernel
        self._toggles = {
            "Sovereign_Mode": True,
            "AI_Core_Active": True,
            "Night_Shift": False,
            "Focus_Mode": False,
            "Quantum_Shield": True,
            "Energy_Efficiency": "Adaptive",
            "Auto_Update": False
        }

    def toggle_state(self, key: str) -> str:
        """USP: Atomic state change for critical OS switches."""
        if key not in self._toggles:
            return "Error: Unknown toggle in Sovereign Registry."
        
        current = self._toggles[key]
        if isinstance(current, bool):
            self._toggles[key] = not current
        else:
            return f"Error: '{key}' is a multi-state parameter. Use set_value()."
            
        return f"ControlCenter: '{key}' is now {'ON' if self._toggles[key] else 'OFF'}."

    def set_value(self, key: str, value: Any) -> str:
        """USP: Set hierarchical values for OS performance tiers."""
        if key not in self._toggles:
            return "Error: Key not found."
        self._toggles[key] = value
        return f"ControlCenter: Set '{key}' to '{value}'."

    def get_quick_stats(self) -> Dict:
        """Returns condensed OS status for the fly-out UI."""
        return {
            "Toggles": self._toggles,
            "Volume": "85%",
            "Brightness": "70%",
            "Network": "Offline-Guard Active",
            "Performance_Mode": self.kernel.modes.get_active_profile()["Mode"]
        }

    def health_check(self) -> str:
        return f"OK — {len(self._toggles)} system toggles monitored."
