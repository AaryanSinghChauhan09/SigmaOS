from abc import ABC, abstractmethod

from ._base import ISchedulingStrategy

class ISchedulingStrategy:
    @abstractmethod
    def select_next(self, queue: list) -> str:
        raise NotImplementedError