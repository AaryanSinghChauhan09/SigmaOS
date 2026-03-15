# Generated method: AutomationEngine._prune_orphan_caches
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    def _prune_orphan_caches(self):
        if hasattr(self.kernel, 'cache'):
            self.kernel.cache.invalidate('temp')