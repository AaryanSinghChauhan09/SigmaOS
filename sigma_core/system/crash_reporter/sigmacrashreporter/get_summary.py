# Generated method: SigmaCrashReporter.get_summary
import time
import uuid
import threading
import json
from typing import Dict, List, Any

class SigmaCrashReporter:
    def get_summary(self) -> Dict:
        return {'total_crashes': len(self._reports), 'recurrent_issues': [m for m, c in self._module_crash_map.items() if c >= self._recurrent_threshold], 'forensic_status': 'LOCKED_IN_SIGMAFS'}