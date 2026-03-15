from abc import ABC, abstractmethod

class SystemBus:
    def detach(self, observer: IObserver):
        self._observers.remove(observer)