# Generated method: SigmaPackageManager.uninstall_package
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SigmaPackageManager:
    def uninstall_package(self, pkg_name: str):
        reg = self._get_registry()
        if pkg_name in reg['apps']:
            shutil.rmtree(reg['apps'][pkg_name]['path'], ignore_errors=True)
            del reg['apps'][pkg_name]
            self._save_registry(reg)
            return True
        return False