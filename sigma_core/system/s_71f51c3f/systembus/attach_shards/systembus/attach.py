from abc import ABC, abstractmethod

class SystemBus:
    def attach(self, observer: IObserver):
        if observer not in self._observers:
            self._observers.append(observer)