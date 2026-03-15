"""
Auto-split from sigma_core\system\automation_engine.py — AutomationEngine._engage_eco_throttling
"""

import time
import threading
from typing import Dict, Any, List, Callable, Optional



class AutomationEngine:
    def _engage_eco_throttling(self):
        if hasattr(self.kernel, 'governor'):
            self.kernel.governor._apply_profile('ECO')
