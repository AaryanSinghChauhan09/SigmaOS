from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading

from ._base import DeviceManager

class DeviceManager:
    def initialize(self):
        pass