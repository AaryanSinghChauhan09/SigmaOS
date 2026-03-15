from abc import ABC, abstractmethod
from ..system_factory import get_factory

class SecurityProxy:
    def access(self, user_id):
        if self._policy.authorize(user_id, str(self._resource)):
            return self._resource
        raise PermissionError('Zero-Trust Violation: Access Denied.')