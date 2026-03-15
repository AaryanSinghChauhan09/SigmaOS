# Generated method: SigmaOmniSearch.universal_search
import os
import sys
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaOmniSearch:
    def universal_search(self, query: str, root_path: str='.') -> Dict[str, Any]:
        """Combined file + module search — the single entry point for all queries."""
        if not self._file_index:
            self.build_index(root_path)
        elapsed_ms = float(int((time.time() - self._last_indexed) * 100000)) / 100.0
        return {'files': self.search_files(query), 'modules': self.search_modules(query), 'query': query, 'time_ms': elapsed_ms}