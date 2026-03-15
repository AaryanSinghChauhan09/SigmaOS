# Generated method: TelemetryVisualizer.log_metric
from typing import List
import math
import random

class TelemetryVisualizer:
    def log_metric(self, value: float):
        self.history.append(value)
        if len(self.history) > 50:
            self.history.pop(0)