"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.trigger_auto_switch
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def trigger_auto_switch(self, app_name: str) -> Dict[str, str]:
        """USP: Automatically profiles an app launch and seamlessly drops into the perfect Mode."""
        lower_app = app_name.lower()
        target_mode = 'Standard'
        for key, mode in self._app_heuristics.items():
            if key in lower_app:
                target_mode = mode
                break
        if target_mode != self._current_mode:
            self.switch_mode(target_mode)
            return {'status': 'Switched', 'from': self._current_mode, 'to': target_mode, 'app': app_name}
        return {'status': 'Unchanged', 'mode': self._current_mode}
