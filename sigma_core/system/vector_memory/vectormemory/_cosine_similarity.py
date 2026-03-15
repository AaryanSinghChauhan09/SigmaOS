# Generated method: VectorMemory._cosine_similarity
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def _cosine_similarity(self, v1, v2) -> float:
        dot = sum((a * b for a, b in zip(v1, v2)))
        return dot