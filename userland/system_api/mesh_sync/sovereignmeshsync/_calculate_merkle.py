# Generated method: SovereignMeshSync._calculate_merkle
import os
import shutil
import hashlib
from pathlib import Path
from typing import List, Dict

class SovereignMeshSync:
    def _calculate_merkle(self, folder_path: Path) -> str:
        """Simple deterministic hash of folder state."""
        hashes = []
        for root, dirs, files in os.walk(folder_path):
            for f in sorted(files):
                hashes.append(hashlib.md5(f.encode()).hexdigest())
        return hashlib.sha256(''.join(hashes).encode()).hexdigest()