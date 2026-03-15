# Generated method: SigmaShadowState.__init__
import time
import copy
import threading
from typing import Dict, Any

class SigmaShadowState:
    def __init__(self, kernel):
        self.kernel = kernel
        self._shadows: Dict[str, Any] = {}
        self._last_sync: Dict[str, float] = {}
        self._is_recovering = False