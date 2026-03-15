# Generated method: SovereignInterruptManager.setup_idt
from enum import Enum, auto
import secrets

class SovereignInterruptManager:
    def setup_idt(self):
        """USP: Interrupt Descriptor Table (ISR Mapping)."""
        self.idt[0] = self._handle_divide_by_zero
        self.idt[8] = self._handle_double_fault
        self.idt[13] = self._handle_gpf
        self.idt[14] = self._handle_page_fault
        self.idt[32] = self._handle_timer_tick
        self.idt[33] = self._handle_keyboard_irq
        self.idt[128] = self._handle_syscall