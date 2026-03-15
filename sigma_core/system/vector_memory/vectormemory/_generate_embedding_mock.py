# Generated method: VectorMemory._generate_embedding_mock
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def _generate_embedding_mock(self, text: str) -> array.array:
        """Simulates a lightweight embedding using character frequency and hashing."""
        emb = [0.0] * self.dim
        for i, char in enumerate(text):
            idx = ord(char) * (i + 1) % self.dim
            emb[idx] += 1.0
        norm = math.sqrt(sum((x * x for x in emb))) or 1.0
        return array.array('f', [x / norm for x in emb])