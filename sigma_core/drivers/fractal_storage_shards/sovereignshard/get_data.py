from functools import lru_cache
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.storage_interfaces import IRedundancyController, IDataShard
import hashlib

from ._base import SovereignShard

class SovereignShard:
    def get_data(self):
        return self._data