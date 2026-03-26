from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.driver_interfaces import IDiskDriver

from ._base import VirtualDiskDriver

class VirtualDiskDriver:
    def read(self, address, length) -> bytes:
        print(f'[DISK] Reading {length} bytes from {address}')
        return bytes(self._storage[address:address + length])