# Generated method: SigmaAuralis.stop_listening
import os
import sys
import threading
import time
import json
import subprocess
from pathlib import Path

class SigmaAuralis:
    def stop_listening(self):
        self.is_listening = False
        self._ready = False