"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.resume_active_timer
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def resume_active_timer(self):
        """Resumes a paused timer."""
        if self._active_timer_task and self._active_timer_paused_at:
            pause_duration = time.time() - self._active_timer_paused_at
            self._active_timer_start += pause_duration
            self._active_timer_paused_at = 0
            if self.kernel:
                self.kernel.bus.emit('projects.timer_resumed', {'tid': self._active_timer_task})
            return True
        return False
