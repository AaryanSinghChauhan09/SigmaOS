from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import ABC, abstractmethod
from ..icommand._base import ICommand

class ICommander(ISystemComponent):
    """
    Commander Interface for SigmaOS logic routing.
    """