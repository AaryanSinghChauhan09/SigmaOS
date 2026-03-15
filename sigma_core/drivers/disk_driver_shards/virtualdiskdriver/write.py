from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver

from ._base import VirtualDiskDriver

class VirtualDiskDriver:
    def write(self, address, data: bytes) -> bool:
        print(f'[DISK] Writing {len(data)} bytes to {address}')
        self._storage[address:address + len(data)] = data
        return True