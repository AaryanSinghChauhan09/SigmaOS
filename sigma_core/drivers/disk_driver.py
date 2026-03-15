from ..interfaces.base_sovereign import SovereignModule
from ..interfaces.driver_interfaces import IDiskDriver

class VirtualDiskDriver(SovereignModule, IDiskDriver):
    """
    Virtual Disk Driver implementation.
    Abstraction of a block storage device.
    """
    def __init__(self, size_kb=1024):
        super().__init__("VIRTUAL_DISK_DRIVER")
        self._storage = bytearray(size_kb * 1024)

    def read(self, address, length) -> bytes:
        print(f"[DISK] Reading {length} bytes from {address}")
        return bytes(self._storage[address:address+length])

    def write(self, address, data: bytes) -> bool:
        print(f"[DISK] Writing {len(data)} bytes to {address}")
        self._storage[address:address+len(data)] = data
        return True

    def flush(self):
        print("[DISK] Flushing cache to virtual NAND.")

    def get_hardware_info(self) -> dict:
        return {"type": "Sovereign_Virtual_NVMe", "capacity": len(self._storage)}

    def execute(self, *args, **kwargs):
        pass

    def initialize(self):
        print("[DISK] Initializing Virtual Disk Shards...")

    def shutdown(self):
        self.flush()
        print("[DISK] Device Offline.")

    def health_check(self) -> bool:
        return True
