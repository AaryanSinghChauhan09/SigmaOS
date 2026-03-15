# Generated method: SigmaSyncSentinel.stop_service
import os
import sys
import time
import threading
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncSentinel:
    def stop_service(self):
        self._sync_active = False