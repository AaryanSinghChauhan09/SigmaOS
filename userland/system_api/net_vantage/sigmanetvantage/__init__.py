# Generated method: SigmaNetVantage.__init__
import os
import sys
import socket
import subprocess
import time
import platform
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaNetVantage:
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self.stats: Dict[str, Any] = {'scans': 0, 'dns_latency_ms': 0.0}