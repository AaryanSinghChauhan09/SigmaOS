"""
Auto-split from sigma_core\system\automation_engine.py — AutomationEngine._purge_redundant_journals
"""

import time
import threading
from typing import Dict, Any, List, Callable, Optional



class AutomationEngine:
    def _purge_redundant_journals(self):
        if hasattr(self.kernel, 'fs'):
            if hasattr(self.kernel.fs, 'flush_intent_log'):
                self.kernel.fs.flush_intent_log()
