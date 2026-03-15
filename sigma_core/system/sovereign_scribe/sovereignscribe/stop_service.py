# Generated method: SovereignScribe.stop_service
import time
import json
import os
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SovereignScribe:
    def stop_service(self) -> None:
        self._running = False
        self._flush_buffer()