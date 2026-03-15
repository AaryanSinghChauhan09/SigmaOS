from ..interfaces.base_sovereign import SovereignModule
from ..interfaces.driver_interfaces import IDeviceDriver
import threading

class DeviceManager(SovereignModule):
    """
    Device Manager (Sovereign Unit).
    Orchestrates hardware drivers using the Registry Pattern.
    """
    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        with cls._lock:
            if cls._instance is None:
                cls._instance = super(DeviceManager, cls).__new__(cls)
                cls._instance._drivers = {}
        return cls._instance

    def __init__(self):
        # Prevent re-initialization if already initialized via Singleton __new__
        if not hasattr(self, 'name'):
            super().__init__("DEVICE_MANAGER")

    def register_driver(self, device_id: str, driver: IDeviceDriver):
        print(f"[DEVICE_MNGR] Registering Driver for: {device_id}")
        self._drivers[device_id] = driver
        driver.initialize()

    def get_driver(self, device_id: str) -> IDeviceDriver:
        return self._drivers.get(device_id)

    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract for the manager."""
        if action == "LIST_DEVICES":
            return list(self._drivers.keys())
        return None

    def initialize(self):
        pass

    def shutdown(self):
        for driver in self._drivers.values():
            driver.shutdown()

    def health_check(self) -> bool:
        return all(d.health_check() for d in self._drivers.values())

def get_device_manager():
    return DeviceManager()
