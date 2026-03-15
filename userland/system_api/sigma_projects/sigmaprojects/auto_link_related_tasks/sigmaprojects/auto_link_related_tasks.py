# Generated method: SigmaProjects.auto_link_related_tasks
import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any

class SigmaProjects:
    def auto_link_related_tasks(self):
        """AI-driven task association."""
        for tid1, t1 in self._tasks.items():
            for tid2, t2 in self._tasks.items():
                if tid1 == tid2:
                    continue
                words1 = set(t1.title.lower().split() + t1.description.lower().split())
                words2 = set(t2.title.lower().split() + t2.description.lower().split())
                if len(words1.intersection(words2)) > 2:
                    self.add_link(tid1, tid2, 'related')