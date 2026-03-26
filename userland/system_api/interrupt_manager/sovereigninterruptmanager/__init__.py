# Generated method: SovereignInterruptManager.__init__
from enum import Enum, auto
import secrets

class SovereignInterruptManager:
    def __init__(self, kernel):
        self.kernel = kernel
        self.gdt: dict[int, SegmentDescriptor] = {}
        self.idt: dict[int, callable] = {}
        self._cpu_state = {'privilege': PrivilegeLevel.RING0}
        self.setup_gdt()
        self.setup_idt()