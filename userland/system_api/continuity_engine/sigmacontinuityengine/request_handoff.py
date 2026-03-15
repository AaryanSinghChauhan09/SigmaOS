# Generated method: SigmaContinuityEngine.request_handoff
from typing import Dict, List, Any
import time

class SigmaContinuityEngine:
    def request_handoff(self, app_id: str, state_data: Dict) -> str:
        """USP: Pick up exactly where you left off on another device."""
        self._handoff_state[app_id] = {'time': time.time(), 'state': state_data}
        return f"Continuity: Handoff data for '{app_id}' staged for broadcast."