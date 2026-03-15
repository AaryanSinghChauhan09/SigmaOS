from abc import ABC, abstractmethod

class GenericScheduler:
    def add_task(self, task):
        self._queue.append(task)