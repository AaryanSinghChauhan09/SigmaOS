from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver

from ._base import VirtualDiskDriver

class VirtualDiskDriver:
    def get_hardware_info(self) -> dict:
        return {'type': 'Sovereign_Virtual_NVMe', 'capacity': len(self._storage)}