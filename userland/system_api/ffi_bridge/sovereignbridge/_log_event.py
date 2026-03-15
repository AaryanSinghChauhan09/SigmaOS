# Generated method: SovereignBridge._log_event
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def _log_event(self, source, msg):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit('system.event', {'source': source, 'message': msg, 'level': 'INFO'})
        print(f'[{source}] {msg}')