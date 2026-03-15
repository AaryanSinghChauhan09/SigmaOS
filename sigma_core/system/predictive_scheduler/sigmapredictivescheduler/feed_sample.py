# Generated method: SigmaPredictiveScheduler.feed_sample
import time
import threading
import collections
import math
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaPredictiveScheduler:
    def feed_sample(self, pid: str, cpu_pct: float):
        with self._lock:
            sig = self._signals.get(pid)
            if not sig:
                return
            sig.update(cpu_pct)
            now = time.time()
            pred = sig.predict(steps=3)
            confidence = sig.level_confidence
            if pred > _BURST_THRESHOLD and confidence > 0.85 and (not sig.pre_boosted):
                self._apply_burst_params(pid, sig)
            elif sig.pre_boosted and cpu_pct < _BURST_THRESHOLD * 0.4 and (now > sig.boost_until):
                self._release_burst_params(pid, sig)