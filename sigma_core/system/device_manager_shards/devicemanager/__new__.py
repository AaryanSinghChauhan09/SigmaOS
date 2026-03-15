from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading

from ._base import DeviceManager

class DeviceManager:
    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(DeviceManager, cls).__new__(cls)
                cls._instance._drivers = {}
        return cls._instance