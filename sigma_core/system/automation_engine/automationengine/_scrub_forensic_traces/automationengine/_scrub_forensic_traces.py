# Generated method: AutomationEngine._scrub_forensic_traces
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    def _scrub_forensic_traces(self):
        if hasattr(self.kernel, 'scrubber'):
            self.kernel.scrubber.scrub_all()