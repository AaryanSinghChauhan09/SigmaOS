# Generated method: SigmaAgentSandbox.__init__
import os
import shutil
import uuid
import time
import subprocess
import threading
from typing import Dict, Any, List

class SigmaAgentSandbox:
    def __init__(self, kernel):
        self.kernel = kernel
        self.base_dir = os.path.join(os.getcwd(), 'userland', 'silos', 'agents')
        self._active_silos: Dict[str, Dict] = {}
        if not os.path.exists(self.base_dir):
            os.makedirs(self.base_dir, exist_ok=True)