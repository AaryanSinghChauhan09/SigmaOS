from abc import ABC, abstractmethod

from ._base import IPrivacyGuard

class IPrivacyGuard:
    @abstractmethod
    def authorize_access(self, data_tag: str, requester_purpose: str) -> bool:
        raise NotImplementedError