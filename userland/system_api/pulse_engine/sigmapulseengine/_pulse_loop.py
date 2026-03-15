# Generated method: SigmaPulseEngine._pulse_loop
from typing import Dict, Any
import time
import threading

class SigmaPulseEngine:
    def _pulse_loop(self):
        while self._is_pulsing:
            self._sync_sovereign_mesh()
            self._listen_ambient_aura()
            time.sleep(1.0 / self._heartbeat_hz)