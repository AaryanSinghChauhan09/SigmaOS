from abc import ABC, abstractmethod

from ._base import IScheduler

class IScheduler:
    @abstractmethod
    def schedule_task(self, task_id, priority):
        raise NotImplementedError