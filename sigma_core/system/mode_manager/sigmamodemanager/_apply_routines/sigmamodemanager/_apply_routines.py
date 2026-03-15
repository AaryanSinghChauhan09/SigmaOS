# Generated method: SigmaModeManager._apply_routines
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def _apply_routines(self, routine_names: List[str], phase: str) -> Dict[str, str]:
        """Executes a list of routines."""
        results = {}
        for routine_name in routine_names:
            results[routine_name] = self._execute_routine(routine_name, phase)
        return results