# Generated method: SigmaOmniAutomator.shutdown
from typing import Callable, Dict, List, Any, Optional
import threading
import time
import random
import uuid

class SigmaOmniAutomator:
    def shutdown(self):
        self._running = False
        return 'OmniAutomator: Offline.'