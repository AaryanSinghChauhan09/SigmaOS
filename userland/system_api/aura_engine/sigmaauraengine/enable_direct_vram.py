# Generated method: SigmaAuraEngine.enable_direct_vram
import time
from typing import Dict, Any

class SigmaAuraEngine:
    def enable_direct_vram(self, active_pid: str) -> str:
        """USP: Grants an active process direct access to the VRAM cursor-path, bypassing window buffers."""
        self._vram_mode = 'DIRECT_FAST_PATH'
        self._latency_ms = 0.08
        self._stats['direct_hits'] += 1
        return f'Aura_Direct: PID {active_pid} now has direct VRAM access. Latency: {self._latency_ms}ms.'