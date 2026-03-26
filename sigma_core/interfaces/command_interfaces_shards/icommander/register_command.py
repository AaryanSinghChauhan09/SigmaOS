from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import ABC, abstractmethod
from ..icommand._base import ICommand
from ._base import ICommander

class ICommander:
    @abstractmethod
    def register_command(self, name: str, command: ICommand):
        raise NotImplementedError