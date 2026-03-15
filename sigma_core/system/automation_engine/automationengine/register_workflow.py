"""
Auto-split from sigma_core\system\automation_engine.py — AutomationEngine.register_workflow
"""

import time
import threading
from typing import Dict, Any, List, Callable, Optional



class AutomationEngine:
    def register_workflow(self, name: str, steps: List[Callable]):
        self.workflows[name] = steps
