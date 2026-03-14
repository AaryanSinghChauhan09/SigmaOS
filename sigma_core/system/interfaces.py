from abc import ABC, abstractmethod
from typing import Dict, Any, Optional

class ISigmaModule(ABC):
    @abstractmethod
    def get_module_id(self) -> str:
        pass

    @abstractmethod
    def health_check(self) -> str:
        pass

class ISigmaService(ISigmaModule):
    @abstractmethod
    def start_service(self) -> str:
        pass

    @abstractmethod
    def stop_service(self) -> None:
        pass

class SigmaModuleBase(ISigmaModule):
    def __init__(self, kernel):
        self.kernel = kernel

    def get_module_id(self) -> str:
        return self.__class__.__name__.lower()

    def health_check(self) -> str:
        return "OK"

    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, "ledger"):
            self.kernel.ledger.commit(self.get_module_id(), action, context)
