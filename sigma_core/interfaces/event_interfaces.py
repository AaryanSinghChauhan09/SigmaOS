from abc import ABC, abstractmethod

class IEventObserver(ABC):
    """
    Observer Pattern Interface.
    Any module interested in system events must implement this.
    """
    @abstractmethod
    def on_event(self, event_type: str, data: dict):
        raise NotImplementedError

class IEventBus(ABC):
    """
    Interface for the Event Bus.
    """
    @abstractmethod
    def subscribe(self, event_type: str, observer: IEventObserver):
        raise NotImplementedError

    @abstractmethod
    def publish(self, event_type: str, data: dict):
        raise NotImplementedError
