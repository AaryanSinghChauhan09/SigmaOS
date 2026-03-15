# Generated method: SovereignMeshSync.__init__
import os
import shutil
import hashlib
from pathlib import Path
from typing import List, Dict

class SovereignMeshSync:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.watched_folders: List[Path] = []
        self._sync_history: List[Dict[str, str]] = []
        self._connected_peers: List[str] = []