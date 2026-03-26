from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import MetricsDecorator

class MetricsDecorator:
    def __init__(self, component):
        super().__init__(component)
        self._exec_count = 0
        self._total_latency = 0.0