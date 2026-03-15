from abc import ABC, abstractmethod

class GenericScheduler:
    def __init__(self, strategy: SchedulingStrategy):
        self._strategy = strategy
        self._queue = []