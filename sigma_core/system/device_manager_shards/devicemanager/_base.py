from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDeviceDriver
import threading


class DeviceManager(SovereignModule):
    """
    Device Manager (Sovereign Unit).
    Orchestrates hardware drivers using the Registry Pattern.
    """
    _instance = None
    _lock = threading.Lock()