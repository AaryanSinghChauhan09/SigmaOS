# Generated method: AdaptiveEnergyController._get_thermal_status
import time
import random
import threading
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from sigma_core.hal.hal import SigmaHAL

class AdaptiveEnergyController:
    def _get_thermal_status(self, temp: float) -> str:
        if temp < 50:
            return 'COOL'
        if temp < 70:
            return 'OPTIMAL'
        if temp < 78:
            return 'WARM'
        if temp < 84:
            return 'THROTTLE'
        return 'CRITICAL'