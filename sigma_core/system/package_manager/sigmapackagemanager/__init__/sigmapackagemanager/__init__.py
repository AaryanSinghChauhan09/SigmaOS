# Generated method: SigmaPackageManager.__init__
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.root = Path(os.path.abspath(os.path.join(os.path.dirname(__file__), '../..')))
        self.registry_path = self.root / 'ecosystem' / 'registry.json'
        self.apps_dir = self.root / 'ecosystem' / 'apps'
        if not self.registry_path.exists():
            self._init_registry()