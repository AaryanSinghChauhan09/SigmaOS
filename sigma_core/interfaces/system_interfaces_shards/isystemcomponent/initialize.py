from abc import ABC, abstractmethod

from ._base import ISystemComponent

class ISystemComponent:
    @abstractmethod
    def initialize(self):
        raise NotImplementedError