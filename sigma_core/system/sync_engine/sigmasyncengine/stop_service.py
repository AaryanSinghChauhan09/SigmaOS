# Generated method: SigmaSyncEngine.stop_service
import time
import json
import threading
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncEngine:
    def stop_service(self):
        self._running = False
        self.log_event('sync_stop', {'status': 'INACTIVE'})