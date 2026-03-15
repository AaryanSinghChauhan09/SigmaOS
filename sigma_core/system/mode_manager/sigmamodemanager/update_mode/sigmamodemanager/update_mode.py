# Generated method: SigmaModeManager.update_mode
from typing import Dict, List, Any, Callable, Optional
import time

class SigmaModeManager:
    def update_mode(self, mode_name: str, config: Dict) -> Dict:
        if mode_name not in self._modes:
            return {'Error': f"Mode '{mode_name}' not found."}
        self._modes[mode_name].update(config)
        return {'Status': f"Mode '{mode_name}' updated successfully."}