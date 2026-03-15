# Generated method: SigmaSyncSentinel.__init__
import os
import sys
import time
import threading
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncSentinel:
    def __init__(self, kernel):
        super().__init__(kernel)
        self._sync_active = True
        self._sync_lock = threading.Lock()
        self._file_hashes: Dict[str, float] = {}
        self._root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))