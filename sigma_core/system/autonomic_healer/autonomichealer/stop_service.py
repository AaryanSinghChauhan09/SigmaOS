# Generated method: AutonomicHealer.stop_service
import threading
import time
import random
from typing import Dict, Any, Optional, List
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class AutonomicHealer:
    def stop_service(self):
        self._running = False