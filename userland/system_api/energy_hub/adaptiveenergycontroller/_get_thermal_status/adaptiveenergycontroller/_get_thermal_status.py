# Generated method: AdaptiveEnergyController._get_thermal_status
import time
import random
import threading
from typing import Dict, Any

class AdaptiveEnergyController:
    def _get_thermal_status(self) -> str:
        t = self.temp_cpu
        if t < 50:
            return 'COOL'
        if t < 70:
            return 'OPTIMAL'
        if t < 78:
            return 'WARM'
        if t < 84:
            return 'THROTTLE'
        return 'CRITICAL'