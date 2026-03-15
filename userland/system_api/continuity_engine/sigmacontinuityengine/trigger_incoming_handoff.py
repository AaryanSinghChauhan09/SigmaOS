# Generated method: SigmaContinuityEngine.trigger_incoming_handoff
from typing import Dict, List, Any
import time

class SigmaContinuityEngine:
    def trigger_incoming_handoff(self, device_name: str, app_name: str, app_icon: str):
        """Simulates an incoming handoff request from another device."""
        self._incoming_handoffs.append({'device': device_name, 'app': app_name, 'icon': app_icon, 'timestamp': time.time()})