from abc import ABC, abstractmethod


class IEventObserver(ABC):
    """
    Observer Pattern Interface.
    Any module interested in system events must implement this.
    """