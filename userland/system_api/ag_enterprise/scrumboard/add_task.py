"""
Auto-split from userland\system_api\ag_enterprise.py — ScrumBoard.add_task
"""

import os
import re
import json
import time
from typing import List, Dict, Any, Optional



class ScrumBoard:
    def add_task(self, title: str, priority: str='Medium'):
        self.tasks.append({'title': title, 'priority': priority, 'status': 'To Do'})
        return f"Scrum: Task '{title}' locked into {priority} orbit."
