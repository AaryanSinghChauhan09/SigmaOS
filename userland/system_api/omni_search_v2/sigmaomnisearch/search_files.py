# Generated method: SigmaOmniSearch.search_files
import os
import sys
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaOmniSearch:
    def search_files(self, query: str) -> List[Dict[str, str]]:
        """Fuzzy file search across the built index."""
        self.stats['queries'] = int(self.stats['queries']) + 1
        q = query.lower()
        results: List[Dict[str, str]] = []
        for name, path in self._file_index.items():
            if q in name:
                results.append({'name': name, 'path': path})
        bounded: List[Dict[str, str]] = []
        for r in results:
            if len(bounded) >= 20:
                break
            bounded.append(r)
        return bounded