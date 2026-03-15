"""
Auto-split from userland\system_api\sigma_projects.py — SigmaProjects.add_link
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Dict, List, Any



class SigmaProjects:
    def add_link(self, tid1: str, tid2: str, link_type='blocks'):
        """Bidirectional link between tasks (Obsidian style)."""
        if tid1 in self._tasks and tid2 in self._tasks:
            self._task_links.append((tid1, tid2, link_type))
            return True
        return False
