# Generated method: SigmaOmniSearch.__init__
import os
import sys
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaOmniSearch:
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self._file_index: Dict[str, str] = {}
        self._last_indexed: float = 0.0
        self.stats = {'queries': 0, 'indexed_files': 0}