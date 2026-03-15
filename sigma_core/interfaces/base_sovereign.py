from abc import ABC, abstractmethod
import time

class ISovereign(ABC):
    @abstractmethod
    def execute(self, *args, **kwargs): pass
    @property
    @abstractmethod
    def metadata(self) -> dict: pass

class SigmaObject(ABC):
    def __init__(self):
        self._created_at = time.time()
        self._id = id(self)
        self._status = "INITIALIZING"

class SovereignModule(SigmaObject, ISovereign):
    def __init__(self, name):
        super().__init__()
        self.name = name
        self._status = "READY"
    @abstractmethod
    def execute(self, *args, **kwargs): pass
    @property
    def metadata(self) -> dict:
        return {"name": self.name, "status": self._status}
