# Generated method: SigmaAuraShield._calculate_entropy
import time
import math
import hashlib
import os
import sys
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaAuraShield:
    def _calculate_entropy(self, data: bytes):
        if not data:
            return 0.0
        entropy = 0.0
        freq: Dict[int, int] = {}
        for b in data:
            freq[b] = freq.get(b, 0) + 1
        for f in freq.values():
            p = float(f) / len(data)
            entropy -= p * math.log2(p)
        return float(entropy / 8.0)