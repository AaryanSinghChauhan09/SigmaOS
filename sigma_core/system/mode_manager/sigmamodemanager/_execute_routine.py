"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._execute_routine
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _execute_routine(self, routine_name: str, phase: str) -> str:
        """Executes a single routine by name."""
        if routine_name in self._routines:
            try:
                return self._routines[routine_name](phase=phase)
            except Exception as e:
                return f'Routine failed: {e}'
        else:
            return f"Routine '{routine_name}' not found."
