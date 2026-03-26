from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver


class VirtualDiskDriver(SovereignModule, IDiskDriver):
    __slots__ = ('_storage',)
    '\n    Virtual Disk Driver implementation.\n    Abstraction of a block storage device.\n    '