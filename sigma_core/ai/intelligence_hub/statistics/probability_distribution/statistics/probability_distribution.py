# Generated method: Statistics.probability_distribution
import math
import random
import time
from typing import List, Dict, Any, Optional

class Statistics:
    def probability_distribution(self, data: List[float]) -> Dict[float, float]:
        total = float(len(data))
        return {x: float(data.count(x)) / total for x in set(data)}