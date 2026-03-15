# Generated method: SigmaDeviceManager.stop_service
import time
import uuid
from typing import Dict, List, Any
from .interfaces import SigmaModuleBase, ISigmaService

class SigmaDeviceManager:
    def stop_service(self):
        self.log_event('service_stop', {'id': 'DeviceManager'})