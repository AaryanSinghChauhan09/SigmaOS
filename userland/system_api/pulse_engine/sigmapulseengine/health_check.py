# Generated method: SigmaPulseEngine.health_check
from typing import Dict, Any
import time
import threading

class SigmaPulseEngine:
    def health_check(self) -> str:
        status = 'PULSING' if self._is_pulsing else 'AWAKE'
        return f'OK — State: {status} | Sentinels: {len(self._active_senses)} active.'