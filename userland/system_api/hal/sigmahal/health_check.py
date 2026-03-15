# Generated method: SigmaHAL.health_check
from enum import Enum, auto

class SigmaHAL:
    def health_check(self) -> str:
        s = self._stats
        return f"OK — Host: {self._host_arch.value}, Devices: {self._active_devices}, JIT Inst: {s['translated_instructions']}, IRQs: {s['interrupts_handled']}."