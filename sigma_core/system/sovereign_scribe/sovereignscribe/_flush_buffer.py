# Generated method: SovereignScribe._flush_buffer
import time
import json
import os
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SovereignScribe:
    def _flush_buffer(self):
        self.log_buffer.clear()