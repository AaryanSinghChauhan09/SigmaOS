"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.remove_routine
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def remove_routine(self, routine_name: str) -> Dict[str, str]:
        if routine_name not in self._routines:
            return {'Error': f"Routine '{routine_name}' not found."}
        self._routines.pop(routine_name, None)
        return {'Status': f"Routine '{routine_name}' removed successfully."}
