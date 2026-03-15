"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._log_mode_change
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _log_mode_change(self, phase: str='') -> str:
        """Logs the mode change event."""
        return f'System log: Mode change {phase} for {self._current_mode} at {time.time()}.'
