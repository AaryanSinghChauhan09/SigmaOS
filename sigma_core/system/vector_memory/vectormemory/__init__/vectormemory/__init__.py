# Generated method: VectorMemory.__init__
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def __init__(self, dimension: int=128, storage_path: str='sigma_storage/vector_memory.bin'):
        self.dim = dimension
        self.storage_path = storage_path
        self.vectors = []
        self.metadata = []
        self._ensure_storage()
        self.load()