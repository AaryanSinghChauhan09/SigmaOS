"""
Auto-split from sigma_core\ai\intelligence_hub.py — Statistics.variance
"""

import math
import random
import time
from typing import List, Dict, Any, Optional



class Statistics:
    def variance(self, data: List[float]) -> float:
        m = self.mean(data)
        return sum(((x - m) ** 2 for x in data)) / len(data) if data else 0.0
