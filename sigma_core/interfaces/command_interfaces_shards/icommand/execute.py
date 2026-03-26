from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import ABC, abstractmethod

from ._base import ICommand

class ICommand:
    @abstractmethod
    def execute(self, *args, **kwargs):
        raise NotImplementedError