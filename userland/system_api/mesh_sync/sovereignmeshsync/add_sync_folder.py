# Generated method: SovereignMeshSync.add_sync_folder
import os
import shutil
import hashlib
from pathlib import Path
from typing import List, Dict

class SovereignMeshSync:
    def add_sync_folder(self, path: str):
        p = Path(path).resolve()
        if p.exists() and p not in self.watched_folders:
            self.watched_folders.append(p)
            print(f'[MESH-SYNC] Watching: {p}')
            return True
        return False