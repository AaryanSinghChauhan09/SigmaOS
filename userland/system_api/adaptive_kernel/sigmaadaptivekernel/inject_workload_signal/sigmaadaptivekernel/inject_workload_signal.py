# Generated method: SigmaAdaptiveKernel.inject_workload_signal
import time
import threading
from enum import Enum, auto

class SigmaAdaptiveKernel:
    def inject_workload_signal(self, signal: str) -> str:
        """Feed a process name / keyword into the sensor list."""
        self._sensors.append(signal)
        return f"AdaptiveKernel: Signal '{signal}' injected into workload sensor."