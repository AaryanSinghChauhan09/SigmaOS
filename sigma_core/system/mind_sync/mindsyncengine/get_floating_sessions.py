# Generated method: MindSyncEngine.get_floating_sessions
import time
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase

class MindSyncEngine:
    def get_floating_sessions(self) -> List[str]:
        """USP: Universal Session Discovery."""
        return list(self._active_sessions)