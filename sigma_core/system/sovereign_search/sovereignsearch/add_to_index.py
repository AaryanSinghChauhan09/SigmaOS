# Generated method: SovereignSearch.add_to_index
from typing import List, Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SovereignSearch:
    def add_to_index(self, key: str, data: Any):
        key = key.lower()
        self.index[key] = data
        self.bloom.add(key)