"""
Auto-split from userland\system_api\adaptive_kernel.py — SigmaAdaptiveKernel.stop_autonomous_observer
"""

import time
import threading
from enum import Enum, auto



class SigmaAdaptiveKernel:
    def stop_autonomous_observer(self) -> str:
        self._running = False
        return 'AdaptiveKernel: Observer stopped.'
