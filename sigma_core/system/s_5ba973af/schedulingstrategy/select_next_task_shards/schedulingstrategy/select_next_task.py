from abc import ABC, abstractmethod

class SchedulingStrategy:
    @abstractmethod
    def select_next_task(self, task_queue):
        pass