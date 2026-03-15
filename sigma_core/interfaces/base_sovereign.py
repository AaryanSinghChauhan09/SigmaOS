from abc import ABC, abstractmethod
import time

class ISovereign(ABC):
    """
    Interface Segregation Principle (ISP): 
    The most basic contract for any Sovereign Unit.
    """
    @abstractmethod
    def execute(self, *args, **kwargs):
        pass

    @property
    @abstractmethod
    def metadata(self) -> dict:
        pass

class SigmaObject(ABC):
    """
    Universal Base Class for SigmaOS.
    Implements Encapsulation and Lifecycle management.
    """
    def __init__(self):
        self._created_at = time.time()
        self.__internal_id = id(self) # Truly Private (Name Mangling)
        self._status = "INITIALIZING"

    @property
    def status(self):
        return self._status

    @status.setter
    def status(self, value):
        self._status = value

    @property
    def object_id(self):
        return self.__internal_id

    def __repr__(self):
        return f"<{self.__class__.__name__} id={self.__internal_id}>"

class SovereignModule(SigmaObject, ISovereign):
    """
    Sovereign Module Base.
    Inherits Lifecycle behaviors and Sovereign contracts.
    """
    def __init__(self, name):
        super().__init__()
        self.name = name
        self.status = "READY"

    @abstractmethod
    def execute(self, *args, **kwargs):
        pass

    @property
    def metadata(self) -> dict:
        return {
            "name": self.name,
            "id": self.object_id,
            "status": self.status,
            "uptime": time.time() - self._created_at
        }
