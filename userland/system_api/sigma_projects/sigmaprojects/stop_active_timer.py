"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.stop_active_timer
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def stop_active_timer(self) -> float:
        """Stops the timer and commits hours to the task."""
        if self._active_timer_task and self._active_timer_start > 0:
            if self._active_timer_paused_at:
                self.resume_active_timer()
            elapsed_sec = time.time() - self._active_timer_start
            hours = elapsed_sec / 3600.0
            tid = self._active_timer_task
            if tid != 'GLOBAL':
                self.log_time(tid, hours)
            self._active_timer_task = None
            self._active_timer_start = 0
            if self.kernel:
                self.kernel.bus.emit('projects.timer_stopped', {'tid': tid, 'hours': hours})
            return hours
        return 0.0
