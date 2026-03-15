# Generated method: SigmaProjects.pause_active_timer
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def pause_active_timer(self):
        """Pauses the timer (e.g., during lunch or idle)."""
        if self._active_timer_task and (not self._active_timer_paused_at):
            self._active_timer_paused_at = time.time()
            if self.kernel:
                self.kernel.bus.emit('projects.timer_paused', {'tid': self._active_timer_task})
            return True
        return False