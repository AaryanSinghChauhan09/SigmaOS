from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading

from ._base import DeviceManager

class DeviceManager:
    def __init__(self):
        if not hasattr(self, 'name'):
            super().__init__('DEVICE_MANAGER')