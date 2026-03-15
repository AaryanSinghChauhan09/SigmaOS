# Generated method: SigmaAuraShield._update_baseline
import time
import math
import hashlib
import os
import sys
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaAuraShield:
    def _update_baseline(self, ext: str, entropy: float):
        current = float(self._behavioral_baseline.get(ext, entropy))
        self._behavioral_baseline[ext] = current * 0.95 + entropy * 0.05
        self._trust_count = int(self._trust_count) + 1