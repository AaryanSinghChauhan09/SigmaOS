from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver

from ._base import VirtualDiskDriver

class VirtualDiskDriver:
    def health_check(self) -> bool:
        return True