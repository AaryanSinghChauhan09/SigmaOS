"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.start_active_timer
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def start_active_timer(self, tid: str):
        """Starts a live timer for a specific task."""
        if tid in self._tasks or tid == 'GLOBAL':
            self._active_timer_task = tid
            self._active_timer_start = time.time()
            if self.kernel:
                self.kernel.bus.emit('projects.timer_started', {'tid': tid})
            return True
        return False
