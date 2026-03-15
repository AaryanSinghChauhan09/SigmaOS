from abc import ABC, abstractmethod
from ..system_factory import get_factory

class ZeroTrustPolicy:
    def authorize(self, user_id, resource_id) -> bool:
        print(f'[SECURITY] Zero-Trust Guard: Authorizing {user_id} for {resource_id}...')
        return False