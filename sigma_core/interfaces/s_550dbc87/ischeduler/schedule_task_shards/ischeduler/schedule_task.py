from abc import ABC, abstractmethod

class IScheduler:
    @abstractmethod
    def schedule_task(self, task_id, priority):
        pass