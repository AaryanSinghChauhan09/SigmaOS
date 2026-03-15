# Generated method: SigmaAuraEngine.health_check
import time
from typing import Dict, Any

class SigmaAuraEngine:
    def health_check(self) -> str:
        return f'OK — AuraEngine v3 | Latency: {self._latency_ms}ms | HZ: {self._display_hz} | VRAM: {self._vram_mode}'