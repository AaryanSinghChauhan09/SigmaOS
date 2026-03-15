from abc import ABC, abstractmethod
import time

from ._base import SigmaObject

class SigmaObject:
    @property
    def object_id(self):
        return self.__internal_id