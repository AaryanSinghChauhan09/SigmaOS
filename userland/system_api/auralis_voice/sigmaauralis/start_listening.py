# Generated method: SigmaAuralis.start_listening
import os
import sys
import threading
import time
import json
import subprocess
from pathlib import Path

class SigmaAuralis:
    def start_listening(self):
        """USP: Non-telemetric background voice sentinel."""
        if self.is_listening:
            return
        self.is_listening = True
        self._ready = True
        threading.Thread(target=self._listen_loop, daemon=True).start()
        if self.kernel:
            self.kernel.bus.emit('auralis.status', {'status': 'LISTENING', 'mode': 'Auralis Apex'})
        print("[AURALIS] Neural Ear Active. Listening for 'Sigma'...")