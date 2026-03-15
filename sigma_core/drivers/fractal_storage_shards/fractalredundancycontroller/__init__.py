from functools import lru_cache
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.storage_interfaces import IRedundancyController, IDataShard
import hashlib

from ._base import FractalRedundancyController

class FractalRedundancyController:
    def __init__(self):
        super().__init__('FRACTAL_STORAGE')
        self._nodes = {}