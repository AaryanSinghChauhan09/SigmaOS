from abc import ABC, abstractmethod

class SystemBus:
    def notify(self, event_type, data):
        for observer in self._observers:
            observer.update(event_type, data)