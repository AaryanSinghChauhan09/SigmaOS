# Generated method: SovereignLab._cosine_sim
import time
import math
import hashlib
import statistics
from typing import List, Dict, Any, Optional

class SovereignLab:
    def _cosine_sim(self, v1: List[float], v2: List[float]) -> float:
        dot = sum((a * b for a, b in zip(v1, v2)))
        norm1 = math.sqrt(sum((a * a for a in v1)))
        norm2 = math.sqrt(sum((b * b for b in v2)))
        return dot / (norm1 * norm2) if norm1 * norm2 > 0 else 0