# Generated method: SigmaSovereignOptimizer._purge_dir
import os
import sys
import shutil
import platform
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignOptimizer:
    def _purge_dir(self, directory: str) -> float:
        size_purged = 0.0
        for root, dirs, files in os.walk(directory):
            for file in files:
                fp = os.path.join(root, file)
                try:
                    size_purged += os.path.getsize(fp) / (1024 * 1024)
                    os.remove(fp)
                except:
                    pass
        return size_purged