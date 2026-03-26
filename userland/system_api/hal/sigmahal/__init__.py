# Generated method: SigmaHAL.__init__
from enum import Enum, auto

class SigmaHAL:
    def __init__(self):
        self._host_arch: Architecture = Architecture.ARM64
        self._active_devices: int = 4
        self._power_state: str = 'Performance'
        self._stats = {'translated_instructions': 0, 'qpu_offloads': 0, 'interrupts_handled': 0}