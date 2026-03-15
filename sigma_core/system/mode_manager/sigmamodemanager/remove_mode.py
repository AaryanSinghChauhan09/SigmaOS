"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.remove_mode
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def remove_mode(self, mode_name: str) -> Dict[str, str]:
        if mode_name not in self._modes:
            return {'Error': f"Mode '{mode_name}' not found."}
        if mode_name == self._current_mode:
            return {'Error': f"Cannot remove active mode '{mode_name}'. Switch to another mode first."}
        self._modes.pop(mode_name, None)
        return {'Status': f"Mode '{mode_name}' removed successfully."}
