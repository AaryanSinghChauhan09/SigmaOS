"""
Auto-split from sigma_core\system\automation_engine.py — AutomationEngine._run_ai_fs_healing
"""

import time
import threading
from typing import Dict, Any, List, Callable, Optional



class AutomationEngine:
    def _run_ai_fs_healing(self):
        if hasattr(self.kernel, 'fs'):
            if hasattr(self.kernel.fs, 'self_heal'):
                self.kernel.fs.self_heal()
