# Generated method: SigmaPackageManager._get_registry
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def _get_registry(self):
        with open(self.registry_path, 'r') as f:
            return json.load(f)