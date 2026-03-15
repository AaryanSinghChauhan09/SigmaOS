from abc import ABC, abstractmethod

class ISystemComponent:
    @abstractmethod
    def shutdown(self):
        pass