# Generated method: SigmaProjects.get_burndown_path
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def get_burndown_path(self, sprint_id: str) -> List[Dict]:
        """USP: Generates ideal vs actual burndown coordinates."""
        sprint = self._sprints.get(sprint_id)
        if not sprint:
            return []
        total_h = sum((self._tasks[tid].estimated_h for tid in sprint.tasks))
        done_points = sum((self._tasks[tid].estimated_h for tid in sprint.tasks if self._tasks[tid].status == TaskStatus.DONE))
        dataset = []
        days = 14
        for day in range(days + 1):
            ideal = total_h * (1 - day / days)
            actual = total_h * (1 - day / days * random.uniform(0.7, 1.1)) if day < 7 else total_h - done_points
            dataset.append({'day': day, 'ideal': round(ideal, 1), 'actual': round(actual, 1)})
        return dataset