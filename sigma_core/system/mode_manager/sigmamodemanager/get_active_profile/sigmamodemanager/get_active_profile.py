# Generated method: SigmaModeManager.get_active_profile
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def get_active_profile(self) -> Dict:
        return {'Mode': self._current_mode, 'Config': self._modes[self._current_mode]}