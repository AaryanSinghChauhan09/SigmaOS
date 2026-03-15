from abc import ABC, abstractmethod

class GenericScheduler:
    def set_strategy(self, strategy: SchedulingStrategy):
        self._strategy = strategy