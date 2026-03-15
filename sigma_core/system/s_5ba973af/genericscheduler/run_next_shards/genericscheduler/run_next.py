from abc import ABC, abstractmethod

class GenericScheduler:
    def run_next(self):
        return self._strategy.select_next_task(self._queue)