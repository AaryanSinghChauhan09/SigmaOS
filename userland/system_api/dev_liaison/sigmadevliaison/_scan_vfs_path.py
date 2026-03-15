# Generated method: SigmaDevLiaison._scan_vfs_path
import os
import sys
import subprocess
import time
from typing import List, Dict, Any

class SigmaDevLiaison:
    def _scan_vfs_path(self, path: str):
        if self.vfs:
            files = self.vfs.list_dir(path)
            print(f'  [DEV] VFS-Scan on {path}: {len(files)} entities found.')