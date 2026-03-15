from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import abstractmethod

from ._base import IDeviceDriver

class IDeviceDriver:
    @abstractmethod
    def write(self, address, data: bytes) -> bool:
        pass