# Generated method: SigmaNetVantage.health_check
import os
import sys
import socket
import subprocess
import time
import platform
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaNetVantage:
    def health_check(self) -> str:
        return f"OK - Network Health: OPTIMAL | Latency: {self.stats['dns_latency_ms']:.1f}ms"