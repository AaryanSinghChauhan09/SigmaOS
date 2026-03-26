from abc import ABC, abstractmethod
from ..ieventobserver._base import IEventObserver

class IEventBus(ABC):
    """
    Interface for the Event Bus.
    """