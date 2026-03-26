from abc import ABC, abstractmethod

from ._base import IEventObserver

class IEventObserver:
    @abstractmethod
    def on_event(self, event_type: str, data: dict):
        raise NotImplementedError