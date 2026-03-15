# Generated method: SigmaPackageManager.list_installed
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def list_installed(self):
        return self._get_registry()['apps']