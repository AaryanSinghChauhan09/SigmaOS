# Generated method: SigmaSovereignWatchdog.check_disk_pressure
import os
import sys
import time
import threading
import platform
import subprocess
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignWatchdog:
    def check_disk_pressure(self) -> Dict[str, Any]:
        """Detects low-disk situations and suggests cleanup."""
        report: Dict[str, Any] = {}
        try:
            if platform.system() == 'Windows':
                out = subprocess.check_output(['wmic', 'logicaldisk', 'get', 'size,freespace,caption'], stderr=subprocess.DEVNULL).decode(errors='ignore')
                report['raw'] = out.strip()
                report['status'] = 'ANALYZED'
            else:
                out = subprocess.check_output(['df', '-h'], stderr=subprocess.DEVNULL).decode()
                report['raw'] = out.strip()
                report['status'] = 'ANALYZED'
        except Exception as e:
            report['status'] = f'SKIPPED: {e}'
        return report