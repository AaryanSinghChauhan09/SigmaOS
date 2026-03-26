from abc import ABC, abstractmethod
import time
from ..isovereign._base import ISovereign
from ..sigmaobject._base import SigmaObject

class SovereignModule(SigmaObject, ISovereign):
    __slots__ = ('name', 'status')

    def __init__(self, name='GENERIC_MODULE'):
        super().__init__()
        self.name = name
        self.status = 'ACTIVE'