# Generated method: SovereignBridge._log_error
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def _log_error(self, source, msg):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit('system.error', {'source': source, 'message': msg, 'level': 'CRITICAL'})
        print(f'[{source}] ERROR: {msg}')