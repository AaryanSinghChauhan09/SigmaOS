# Generated method: SigmaSiloManager.__init__
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaSiloManager:
    def __init__(self, kernel):
        self.kernel = kernel
        self.silos: Dict[str, SigmaSilo] = {}
        self._max_resources = {'cpu': 16, 'ram': 32}
        self._used_resources = {'cpu': 0, 'ram': 0}