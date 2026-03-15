"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.ai_sprint_planner
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def ai_sprint_planner(self, capacity_h: float) -> List[str]:
        """USP: AI-driven task selection for optimal sprint packing."""
        backlog = [t for t in self._tasks.values() if t.status == TaskStatus.BACKLOG]
        backlog.sort(key=lambda x: (x.priority.value, -x.estimated_h), reverse=True)
        selected = []
        current_h = 0
        for t in backlog:
            if current_h + t.estimated_h <= capacity_h:
                selected.append(t.task_id)
                current_h += t.estimated_h
        return selected
