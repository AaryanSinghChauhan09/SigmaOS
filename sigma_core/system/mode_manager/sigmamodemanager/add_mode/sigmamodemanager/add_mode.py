# Generated method: SigmaModeManager.add_mode
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def add_mode(self, mode_name: str, config: Dict) -> Dict:
        if mode_name in self._modes:
            return {'Error': f"Mode '{mode_name}' already exists."}
        self._modes[mode_name] = config
        return {'Status': f"Mode '{mode_name}' added successfully."}