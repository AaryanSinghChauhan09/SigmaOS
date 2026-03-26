from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import ABC, abstractmethod


class ICommand(ABC):
    """
    Command Pattern Interface.
    Encapsulates a request as an object.
    """