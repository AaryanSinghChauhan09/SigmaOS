# Generated method: TelemetryVisualizer.get_realtime_stream
from typing import List
import math
import random

class TelemetryVisualizer:
    def get_realtime_stream(self):
        """Mock stream of CPU/Memory usage."""
        val = 10 + 20 * math.sin(random.random()) + random.uniform(0, 40)
        self.log_metric(val)
        return val