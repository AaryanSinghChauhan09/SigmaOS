# Generated method: SigmaModuleBase.get_module_id
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional

class SigmaModuleBase:
    def get_module_id(self) -> str:
        return self.__class__.__name__.lower()