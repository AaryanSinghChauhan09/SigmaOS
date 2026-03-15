# Generated method: EcoManager._check_telemetry
import time
import random
from typing import Dict, Any

class EcoManager:
    def _check_telemetry(self):
        self.system_temp += random.uniform(-2, 5)
        self.low_power_mode = random.choices([True, False], weights=[1, 9])[0]
        print(f"[ECO] System Temp: {self.system_temp:.1f}C | Power Mode: {('ECO' if self.low_power_mode else 'High Perf')}")