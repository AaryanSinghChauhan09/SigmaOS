# Generated method: SigmaOmniAutomator.get_error_log
from typing import Callable, Dict, List, Any, Optional
import threading
import time
import random
import uuid

class SigmaOmniAutomator:
    def get_error_log(self) -> List[str]:
        """Returns the automation error log for diagnostics."""
        return self._error_log.copy()