"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.get_scrum_analytics
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def get_scrum_analytics(self) -> Dict:
        """USP: AI-Assisted BurnDown and Velocity calculation."""
        total_points = sum((t.estimated_h for t in self._tasks.values()))
        done_points = sum((t.estimated_h for t in self._tasks.values() if t.status == TaskStatus.DONE))
        in_prog = sum((t.estimated_h for t in self._tasks.values() if t.status == TaskStatus.IN_PROGRESS))
        health = 100
        if in_prog > total_points * 0.5:
            health -= 20
        if done_points < total_points * 0.1 and time.time() > self._active_timer_start + 172800:
            health -= 15
        return {'velocity': done_points / max(1, len(self._sprints)), 'burndown': total_points - done_points, 'efficiency': done_points / max(0.1, sum((t.actual_h for t in self._tasks.values()))) * 100, 'health_score': max(0, health)}
