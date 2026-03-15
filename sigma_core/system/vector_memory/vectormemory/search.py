# Generated method: VectorMemory.search
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def search(self, query: str, top_k: int=3) -> List[Dict]:
        """Finds most similar memories using Cosine Similarity."""
        if not self.vectors:
            return []
        query_vec = self._generate_embedding_mock(query)
        scores = []
        for i, vec in enumerate(self.vectors):
            score = self._cosine_similarity(query_vec, vec)
            scores.append((score, self.metadata[i]))
        scores.sort(key=lambda x: x[0], reverse=True)
        return [item[1] for item in scores[:top_k]]