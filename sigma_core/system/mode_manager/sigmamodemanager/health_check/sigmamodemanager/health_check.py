# Generated method: SigmaModeManager.health_check
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def health_check(self) -> str:
        return f'OK — Mode: {self._current_mode}.'