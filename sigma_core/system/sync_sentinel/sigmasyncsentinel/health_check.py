# Generated method: SigmaSyncSentinel.health_check
import os
import sys
import time
import threading
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncSentinel:
    def health_check(self) -> str:
        return f'OK - Monitoring {len(self._file_hashes)} files'