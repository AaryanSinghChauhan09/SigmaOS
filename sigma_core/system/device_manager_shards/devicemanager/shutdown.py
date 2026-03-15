from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading

from ._base import DeviceManager

class DeviceManager:
    def shutdown(self):
        for driver in self._drivers.values():
            driver.shutdown()