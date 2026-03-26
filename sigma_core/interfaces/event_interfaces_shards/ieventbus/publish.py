from abc import ABC, abstractmethod

from ._base import IEventBus

class IEventBus:
    @abstractmethod
    def publish(self, event_type: str, data: dict):
        raise NotImplementedError