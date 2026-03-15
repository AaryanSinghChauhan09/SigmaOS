# Generated method: SigmaModuleBase.__init__
import os
import sys
import time
import threading
import queue
from typing import Dict, List, Any, Optional, Callable
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaModuleBase:
    def __init__(self, kernel):
        self.kernel = kernel