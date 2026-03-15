# Generated method: EcoManager._disengage_eco_mode
import time
import random
from typing import Dict, Any

class EcoManager:
    def _disengage_eco_mode(self):
        if self.system_temp < 60.0:
            print('[ECO] Thermal envelope stable. Restoring standard performance.')