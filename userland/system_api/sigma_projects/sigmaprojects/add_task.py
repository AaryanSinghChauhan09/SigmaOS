"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.add_task
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def add_task(self, title, desc='', status=TaskStatus.TODO, priority=Priority.MEDIUM, sprint_id='') -> str:
        tid = f'TSK-{str(uuid.uuid4())[:8]}'
        task = ProjectTask(task_id=tid, title=title, description=desc, status=status, priority=priority, sprint_id=sprint_id, created_at=time.strftime('%Y-%m-%d %H:%M'))
        self._tasks[tid] = task
        if sprint_id in self._sprints:
            self._sprints[sprint_id].tasks.append(tid)
        return tid
