# Generated method: ISigmaModule.health_check
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional

class ISigmaModule:
    @abstractmethod
    def health_check(self) -> str:
        pass