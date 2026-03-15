# Generated method: SigmaPackageManager._init_registry
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def _init_registry(self):
        data = {'apps': {}, 'repositories': ['https://repo.sigmaos.sovereign']}
        self.registry_path.parent.mkdir(parents=True, exist_ok=True)
        with open(self.registry_path, 'w') as f:
            json.dump(data, f, indent=4)