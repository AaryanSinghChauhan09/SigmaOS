from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading
from ..devicemanager._base import DeviceManager

def get_device_manager():
    return DeviceManager()