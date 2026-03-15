# Generated method: ComplianceGuard.stop_service
import os
import time
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class ComplianceGuard:
    def stop_service(self):
        self._running = False