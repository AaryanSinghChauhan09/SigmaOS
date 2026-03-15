# Generated method: SigmaSovereignWatchdog.purge_stale_processes
import os
import sys
import time
import threading
import platform
import subprocess
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignWatchdog:
    def purge_stale_processes(self) -> List[str]:
        """Identifies and lists candidate stale/zombie processes."""
        stale: List[str] = []
        try:
            if platform.system() == 'Windows':
                out = subprocess.check_output(['tasklist', '/FO', 'CSV'], stderr=subprocess.DEVNULL).decode(errors='ignore')
                all_lines = out.splitlines()
                for line in all_lines[1:]:
                    parts = line.split(',')
                    if len(parts) > 4:
                        mem_str = parts[4].replace('"', '').replace(' K', '').replace(',', '').strip()
                        try:
                            if int(mem_str) < 500:
                                stale.append(parts[0].strip('"'))
                        except ValueError:
                            pass
        except Exception:
            pass
        result: List[str] = []
        for s in stale:
            if len(result) >= 10:
                break
            result.append(s)
        return result