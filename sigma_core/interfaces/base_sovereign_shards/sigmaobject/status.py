from abc import ABC, abstractmethod
import time

from ._base import SigmaObject

class SigmaObject:
    @status.setter
    def status(self, value):
        self._status = value