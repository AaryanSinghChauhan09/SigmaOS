# Generated method: SigmaAdaptiveKernel.stop_autonomous_observer
import time
import threading
from enum import Enum, auto

class SigmaAdaptiveKernel:
    def stop_autonomous_observer(self) -> str:
        self._running = False
        return 'AdaptiveKernel: Observer stopped.'