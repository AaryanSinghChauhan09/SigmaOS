# Generated method: SigmaDeviceManager._map_device
import time
import uuid
from typing import Dict, List, Any
from .interfaces import SigmaModuleBase, ISigmaService

class SigmaDeviceManager:
    def _map_device(self, dev_id: str, name: str):
        driver_id = f'drv-{uuid.uuid4().hex[:4]}'
        self.drivers[dev_id] = SovereignDriver(driver_id, name)