# Generated method: SovereignInterruptManager._handle_timer_tick
from enum import Enum, auto
import secrets

class SovereignInterruptManager:
    def _handle_timer_tick(self, p):
        if hasattr(self.kernel, 'scheduler'):
            self.kernel.scheduler.tick()
        return 'TICK'