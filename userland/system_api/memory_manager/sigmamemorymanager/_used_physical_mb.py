"""
Auto-split from userland\system_api\memory_manager.py — SigmaMemoryManager._used_physical_mb
"""

import time
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Any, Dict



class SigmaMemoryManager:
    def _used_physical_mb(self) -> float:
        return sum((e.size_mb * e.compression_ratio for e in self._allocated.values() if e.tier in (MemoryTier.SIGMA_RAM, MemoryTier.ZRAM_CACHE, MemoryTier.NEURAL_NMC)))
