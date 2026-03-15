from abc import ABC, abstractmethod

class ISystemComponent:
    @abstractmethod
    def initialize(self):
        pass