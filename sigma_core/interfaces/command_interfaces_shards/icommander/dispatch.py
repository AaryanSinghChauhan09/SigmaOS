from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import ABC, abstractmethod

from ._base import ICommander

class ICommander:
    @abstractmethod
    def dispatch(self, name: str, *args, **kwargs):
        raise NotImplementedError