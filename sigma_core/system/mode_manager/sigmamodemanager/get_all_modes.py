"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.get_all_modes
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def get_all_modes(self) -> List[str]:
        return list(self._modes.keys())
