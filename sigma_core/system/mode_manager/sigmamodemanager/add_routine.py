"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.add_routine
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def add_routine(self, routine_name: str, routine_func: Callable[..., Any]) -> Dict:
        if routine_name in self._routines:
            return {'Error': f"Routine '{routine_name}' already exists."}
        self._routines[routine_name] = routine_func
        return {'Status': f"Routine '{routine_name}' added successfully."}
