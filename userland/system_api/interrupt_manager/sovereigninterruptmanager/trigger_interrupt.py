# Generated method: SovereignInterruptManager.trigger_interrupt
from enum import Enum, auto
import secrets

class SovereignInterruptManager:
    def trigger_interrupt(self, vector: int, payload: any=None):
        """CPU-like ISR execution."""
        handler = self.idt.get(vector)
        if handler:
            return handler(payload)
        return self._handle_unhandled_irq(vector)