from abc import ABC, abstractmethod

from ._base import ISystemComponent

class ISystemComponent:
    @abstractmethod
    def shutdown(self):
        raise NotImplementedError