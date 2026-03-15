# Generated method: SovereignShell._handle_system
import sys
import os
import time
from typing import List, Optional, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from .interfaces import SigmaModuleBase, ISigmaService

class SovereignShell:
    def _handle_system(self, args: List[str]) -> str:
        if not self.kernel:
            return 'Error: Kernel Offline.'
        if not args or 'health' in args:
            return str(self.kernel.health_check())
        if 'telemetry' in args:
            hal = self.kernel.registry.get('hal')
            return str(hal.get_hardware_state()) if hal else 'HAL missing.'
        return 'Usage: system [health|telemetry]'