# Generated method: IntentEngine.record_action
import time
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase

class IntentEngine:
    def record_action(self, action: str):
        """Records a user interaction for neural analysis."""
        self.history.append({'action': action, 'ts': time.time()})
        if len(self.history) > 100:
            self.history.pop(0)