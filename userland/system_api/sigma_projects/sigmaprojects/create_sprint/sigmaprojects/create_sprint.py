# Generated method: SigmaProjects.create_sprint
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def create_sprint(self, name, goal, start, end) -> str:
        sid = f'SPR-{str(uuid.uuid4())[:6]}'
        sprint = Sprint(sid, name, goal, start, end)
        self._sprints[sid] = sprint
        return sid