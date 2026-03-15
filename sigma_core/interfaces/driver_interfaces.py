from .system_interfaces import ISystemComponent
from abc import abstractmethod

class IDeviceDriver(ISystemComponent):
    """
    Interface for all Low-Level Device Drivers.
    Abstraction over hardware.
    """
    @abstractmethod
    def read(self, address, length) -> bytes:
        pass

    @abstractmethod
    def write(self, address, data: bytes) -> bool:
        pass

    @abstractmethod
    def get_hardware_info(self) -> dict:
        pass

class IDiskDriver(IDeviceDriver):
    @abstractmethod
    def flush(self):
        pass

class INetworkDriver(IDeviceDriver):
    @abstractmethod
    def send_packet(self, target_ip, payload: bytes):
        pass

    @abstractmethod
    def receive_packet(self) -> bytes:
        pass
