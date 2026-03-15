from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import MetricsDecorator

class MetricsDecorator:
    def execute(self, action, *args, **kwargs):
        start = time.time()
        res = super().execute(action, *args, **kwargs)
        latency = time.time() - start
        self._exec_count += 1
        self._total_latency += latency
        print(f"[METRICS] {self.metadata.get('name')} | Total Execs: {self._exec_count} | Latency: {latency:.6f}s")
        return res