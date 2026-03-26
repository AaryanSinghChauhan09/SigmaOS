from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import abstractmethod

from ._base import INetworkDriver

class INetworkDriver:
    @abstractmethod
    def send_packet(self, target_ip, payload: bytes):
        pass