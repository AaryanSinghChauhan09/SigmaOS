# Generated method: SigmaAuralis._listen_loop
import os
import sys
import threading
import time
import json
import subprocess
from pathlib import Path

class SigmaAuralis:
    def _listen_loop(self):
        while self.is_listening:
            time.sleep(15)
            if self.is_listening:
                if self.kernel:
                    self.kernel.bus.emit('auralis.pulse', {'state': 'ready'})