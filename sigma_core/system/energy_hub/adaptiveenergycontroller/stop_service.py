# Generated method: AdaptiveEnergyController.stop_service
import time
import random
import threading
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from sigma_core.hal.hal import SigmaHAL

class AdaptiveEnergyController:
    def stop_service(self) -> None:
        self._running = False