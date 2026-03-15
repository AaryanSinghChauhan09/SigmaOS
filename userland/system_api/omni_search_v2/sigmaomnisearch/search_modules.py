# Generated method: SigmaOmniSearch.search_modules
import os
import sys
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaOmniSearch:
    def search_modules(self, query: str) -> List[str]:
        """Searches loaded SigmaOS kernel modules by name."""
        self.stats['queries'] = int(self.stats['queries']) + 1
        the_kernel = getattr(self, 'kernel', None)
        if not the_kernel or not hasattr(the_kernel, 'registry'):
            return []
        q = query.lower()
        return [k for k in the_kernel.registry.keys() if q in k.lower()]