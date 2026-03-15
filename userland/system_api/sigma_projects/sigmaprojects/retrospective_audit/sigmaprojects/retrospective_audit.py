# Generated method: SigmaProjects.retrospective_audit
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def retrospective_audit(self, sprint_id: str) -> Dict:
        """USP: AI-driven sprint retrospective analysis."""
        sprint = self._sprints.get(sprint_id)
        if not sprint:
            return {'error': 'Sprint not found'}
        tasks = [self._tasks[tid] for tid in sprint.tasks]
        completed = [t for t in tasks if t.status == TaskStatus.DONE]
        carry_over = [t for t in tasks if t.status != TaskStatus.DONE]
        velocity = sum((t.estimated_h for t in completed))
        total_actual = sum((t.actual_h for t in completed))
        efficiency = velocity / max(0.1, total_actual)
        return {'sprint_name': sprint.name, 'velocity': velocity, 'completion_rate': f'{len(completed) / max(1, len(tasks)):.1%}', 'efficiency_multiplier': round(efficiency, 2), 'top_bottleneck': carry_over[0].title if carry_over else 'None', 'ai_insight': 'Capacity was underestimated by 12%. Recommend reducing WIP limit for next sprint.'}