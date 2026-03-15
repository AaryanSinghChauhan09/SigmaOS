from .system_interfaces import ISystemComponent
from abc import ABC, abstractmethod

class ICommand(ABC):
    """
    Command Pattern Interface.
    Encapsulates a request as an object.
    """
    @abstractmethod
    def execute(self, *args, **kwargs):
        raise NotImplementedError

class ICommander(ISystemComponent):
    """
    Commander Interface for SigmaOS logic routing.
    """
    @abstractmethod
    def register_command(self, name: str, command: ICommand):
        raise NotImplementedError

    @abstractmethod
    def dispatch(self, name: str, *args, **kwargs):
        raise NotImplementedError
