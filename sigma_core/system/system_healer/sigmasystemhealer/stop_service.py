# Generated method: SigmaSystemHealer.stop_service
import os
import sys
import threading
import time
import ctypes
import platform
import subprocess
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSystemHealer:
    def stop_service(self) -> None:
        with self._lock:
            self.running = False
        self.log_event('healer_stop', {'status': 'INACTIVE'})