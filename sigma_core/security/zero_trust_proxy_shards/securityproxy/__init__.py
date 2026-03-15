from abc import ABC, abstractmethod
from ..system_factory import get_factory

class SecurityProxy:
    def __init__(self, resource, policy: IAccessControl):
        self._resource = resource
        self._policy = policy