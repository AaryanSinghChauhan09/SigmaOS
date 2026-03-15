# Generated method: VectorMemory.add_memory
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def add_memory(self, text: str, payload: Dict[str, Any]=None):
        """Stores a new contextual memory."""
        vector = self._generate_embedding_mock(text)
        meta = {'text': text, 'timestamp': time.time(), 'payload': payload or {}}
        self.vectors.append(vector)
        self.metadata.append(meta)
        self.save()