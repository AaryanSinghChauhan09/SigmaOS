from abc import ABC, abstractmethod

from ._base import ISystemComponent

class ISystemComponent:
    @abstractmethod
    def health_check(self) -> bool:
        raise NotImplementedError