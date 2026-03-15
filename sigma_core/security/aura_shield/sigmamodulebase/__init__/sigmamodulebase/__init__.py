# Generated method: SigmaModuleBase.__init__
import time
import math
import hashlib
import os
import sys
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaModuleBase:
    def __init__(self, kernel):
        self.kernel = kernel