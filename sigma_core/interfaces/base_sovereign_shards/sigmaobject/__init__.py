from abc import ABC, abstractmethod
import time

from ._base import SigmaObject

class SigmaObject:
    def __init__(self):
        self._created_at = time.time()
        self.__internal_id = id(self)
        self._status = 'INITIALIZING'