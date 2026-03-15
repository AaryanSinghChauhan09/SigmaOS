"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.get_routine_details
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def get_routine_details(self, routine_name: str) -> str:
        if routine_name in self._routines:
            return f"Routine '{routine_name}': {self._routines[routine_name].__doc__ or 'No documentation provided.'}"
        return f"Routine '{routine_name}' not found."
