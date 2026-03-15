# Generated method: SigmaModeManager.get_mode_details
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def get_mode_details(self, mode_name: str) -> Dict:
        return self._modes.get(mode_name, {'Error': f"Mode '{mode_name}' not found."})