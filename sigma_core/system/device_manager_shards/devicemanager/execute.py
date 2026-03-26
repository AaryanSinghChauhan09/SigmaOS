from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading

from ._base import DeviceManager

class DeviceManager:
    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract for the manager."""
        if action == 'LIST_DEVICES':
            return list(self._drivers.keys())
        return None