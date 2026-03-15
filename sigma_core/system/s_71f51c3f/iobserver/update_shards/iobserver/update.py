from abc import ABC, abstractmethod

class IObserver:
    @abstractmethod
    def update(self, event_type, data):
        pass