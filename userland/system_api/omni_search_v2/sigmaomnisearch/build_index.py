# Generated method: SigmaOmniSearch.build_index
import os
import sys
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaOmniSearch:
    def build_index(self, root_path: str, max_depth: int=4) -> int:
        """Builds a fast in-memory file index of the given path."""
        self._file_index = {}
        counter = [0]

        def _walk(path: str, depth: int):
            if depth > max_depth:
                return
            try:
                for entry in os.listdir(path):
                    full = os.path.join(path, entry)
                    if os.path.isfile(full):
                        self._file_index[entry.lower()] = full
                        counter[0] = counter[0] + 1
                    elif os.path.isdir(full) and (not entry.startswith('.')):
                        _walk(full, depth + 1)
            except PermissionError:
                pass
        _walk(root_path, 0)
        self._last_indexed = time.time()
        self.stats['indexed_files'] = counter[0]
        return counter[0]