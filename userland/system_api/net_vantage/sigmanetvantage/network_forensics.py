# Generated method: SigmaNetVantage.network_forensics
import os
import sys
import socket
import subprocess
import time
import platform
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaNetVantage:
    def network_forensics(self) -> List[str]:
        """Scans for active network connections."""
        active_conns = []
        try:
            if platform.system() == 'Windows':
                out = subprocess.check_output(['netstat', '-an']).decode()
                for line in out.splitlines():
                    if 'ESTABLISHED' in line:
                        active_conns.append(line.strip())
        except:
            pass
        return active_conns