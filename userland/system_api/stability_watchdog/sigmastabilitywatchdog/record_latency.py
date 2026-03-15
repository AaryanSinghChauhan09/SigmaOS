# Generated method: SigmaStabilityWatchdog.record_latency
import time
import threading
import collections
from typing import Dict, List, Any

class SigmaStabilityWatchdog:
    def record_latency(self, module_name: str, latency_ms: float):
        if module_name not in self._latencies:
            self._latencies[module_name] = collections.deque(maxlen=100)
        self._latencies[module_name].append(latency_ms)
        sorted_times = sorted(list(self._latencies[module_name]))
        p99 = sorted_times[int(len(sorted_times) * 0.99)] if sorted_times else 0
        if p99 > self._threshold_latency_p99:
            self.kernel.bus.emit('stability.p99_alert', {'module': module_name, 'p99_ms': p99})