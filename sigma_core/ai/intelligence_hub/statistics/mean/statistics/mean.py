# Generated method: Statistics.mean
import math
import random
import time
from typing import List, Dict, Any, Optional

class Statistics:
    def mean(self, data: List[float]) -> float:
        return sum(data) / len(data) if data else 0.0