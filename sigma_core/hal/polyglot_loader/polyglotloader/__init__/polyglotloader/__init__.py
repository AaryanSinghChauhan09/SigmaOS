# Generated method: PolyglotLoader.__init__
import os
import subprocess
import platform
from typing import Dict

class PolyglotLoader:
    def __init__(self):
        self.os_type = platform.system()
        self.bin_path = os.path.join(os.path.dirname(__file__), 'native_bin')
        self.status = {}
        self.active_cores: Dict[str, str] = {}