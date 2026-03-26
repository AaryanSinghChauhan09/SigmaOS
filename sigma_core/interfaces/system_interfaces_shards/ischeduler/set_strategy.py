from abc import ABC, abstractmethod
from ..ischedulingstrategy._base import ISchedulingStrategy
from ._base import IScheduler

class IScheduler:
    @abstractmethod
    def set_strategy(self, strategy: ISchedulingStrategy):
        raise NotImplementedError