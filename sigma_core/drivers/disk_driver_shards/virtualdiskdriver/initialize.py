from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver

from ._base import VirtualDiskDriver

class VirtualDiskDriver:
    def initialize(self):
        print('[DISK] Initializing Virtual Disk Shards...')