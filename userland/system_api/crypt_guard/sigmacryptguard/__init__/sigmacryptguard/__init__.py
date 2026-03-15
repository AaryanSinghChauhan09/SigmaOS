# Generated method: SigmaCryptGuard.__init__
import os
import sys
import hashlib
import binascii
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaCryptGuard:
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self.stats: Dict[str, int] = {'ops': 0}