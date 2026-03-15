from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver

from ._base import VirtualDiskDriver

class VirtualDiskDriver:
    def __init__(self, size_kb=1024):
        super().__init__('VIRTUAL_DISK_DRIVER')
        self._storage = bytearray(size_kb * 1024)