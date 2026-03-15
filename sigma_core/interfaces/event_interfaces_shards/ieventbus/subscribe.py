from abc import ABC, abstractmethod
from ..ieventobserver._base import IEventObserver
from ._base import IEventBus

class IEventBus:
    @abstractmethod
    def subscribe(self, event_type: str, observer: IEventObserver):
        raise NotImplementedError