from abc import ABC, abstractmethod
import time
from ..isovereign._base import ISovereign
from ..sigmaobject._base import SigmaObject

class SovereignModule(SigmaObject, ISovereign):
    __slots__ = ('name', 'status')
    '\n    Sovereign Module Base.\n    Inherits Lifecycle behaviors and Sovereign contracts.\n    '