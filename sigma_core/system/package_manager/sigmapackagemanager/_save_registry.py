# Generated method: SigmaPackageManager._save_registry
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def _save_registry(self, data):
        with open(self.registry_path, 'w') as f:
            json.dump(data, f, indent=4)