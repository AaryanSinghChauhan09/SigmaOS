from abc import ABC, abstractmethod
import time

from ._base import SigmaObject

class SigmaObject:
    def __repr__(self):
        return f'<{self.__class__.__name__} id={self.__internal_id}>'