# Generated method: SovereignShell.__init__
import sys
import os
import time
from typing import List, Optional, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from .interfaces import SigmaModuleBase, ISigmaService

class SovereignShell:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.prompt = 'Σos> '
        self.history: List[str] = []
        self._running = False