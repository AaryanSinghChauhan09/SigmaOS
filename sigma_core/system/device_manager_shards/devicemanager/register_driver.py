from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading

from ._base import DeviceManager

class DeviceManager:
    def register_driver(self, device_id: str, driver: IDeviceDriver):
        print(f'[DEVICE_MNGR] Registering Driver for: {device_id}')
        self._drivers[device_id] = driver
        driver.initialize()