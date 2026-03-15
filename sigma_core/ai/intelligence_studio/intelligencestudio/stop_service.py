# Generated method: IntelligenceStudio.stop_service
import time
import random
import os
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from userland.system_api.sigma_std import SigmaMath

class IntelligenceStudio:
    def stop_service(self):
        self._running = False
        self.log_event('service_stop', {'id': 'IntelligenceStudio'})