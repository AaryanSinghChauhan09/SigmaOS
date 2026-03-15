"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.get_critical_path
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def get_critical_path(self) -> List[str]:
        """USP: Identifies the sequence of tasks that determine project duration."""
        scored = []
        for tid, t in self._tasks.items():
            score = len(t.dependencies) * 10
            if t.priority == Priority.URGENT:
                score += 50
            elif t.priority == Priority.HIGH:
                score += 30
            scored.append((tid, score))
        return [x[0] for x in sorted(scored, key=lambda x: x[1], reverse=True)[:5]]
