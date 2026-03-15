# Generated method: SigmaAuralis.__init__
import os
import sys
import threading
import time
import json
import subprocess
from pathlib import Path

class SigmaAuralis:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.is_listening = False
        self._auralis_lock = threading.Lock()
        self.hotword = 'Sigma'
        self.history = []
        self._last_command = None
        self._ready = False