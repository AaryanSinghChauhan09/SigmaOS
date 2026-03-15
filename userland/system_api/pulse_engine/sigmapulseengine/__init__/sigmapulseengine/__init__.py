# Generated method: SigmaPulseEngine.__init__
from typing import Dict, Any
import time
import threading

class SigmaPulseEngine:
    def __init__(self, kernel):
        self.kernel = kernel
        self._is_pulsing = False
        self._sentinel_thread = None
        self._heartbeat_hz = 1.0
        self._active_senses = ['Aura_Wake', 'Mesh_Sync', 'Security_Watch']