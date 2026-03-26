from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import abstractmethod

from ._base import IDiskDriver

class IDiskDriver:
    @abstractmethod
    def flush(self):
        pass