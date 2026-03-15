"""
Auto-split from sigma_core\ai\intelligence_hub.py — Mathematics.dot_product
"""

import math
import random
import time
from typing import List, Dict, Any, Optional



class Mathematics:
    def dot_product(self, v1: List[float], v2: List[float]) -> float:
        return sum((a * b for a, b in zip(v1, v2)))
