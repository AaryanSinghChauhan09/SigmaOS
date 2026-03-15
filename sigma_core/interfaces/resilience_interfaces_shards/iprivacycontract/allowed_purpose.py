from abc import ABC, abstractmethod

from ._base import IPrivacyContract

class IPrivacyContract:
    @property
    @abstractmethod
    def allowed_purpose(self) -> str:
        raise NotImplementedError