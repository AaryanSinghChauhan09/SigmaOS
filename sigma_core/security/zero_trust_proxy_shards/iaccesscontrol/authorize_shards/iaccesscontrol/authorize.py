from abc import ABC, abstractmethod
from ..system_factory import get_factory

class IAccessControl:
    @abstractmethod
    def authorize(self, user_id, resource_id) -> bool:
        pass