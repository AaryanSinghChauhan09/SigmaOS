# Generated method: VectorMemory.load
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def load(self):
        """Restores memory state from disk."""
        if not os.path.exists(self.storage_path):
            return
        try:
            with open(self.storage_path, 'rb') as f:
                size_data = f.read(4)
                if not size_data:
                    return
                count = int.from_bytes(size_data, 'little')
                for _ in range(count):
                    vec = array.array('f')
                    vec.fromfile(f, self.dim)
                    self.vectors.append(vec)
            meta_path = self.storage_path + '.meta'
            if os.path.exists(meta_path):
                with open(meta_path, 'r') as f:
                    self.metadata = json.load(f)
        except Exception as e:
            print(f'VectorMemory Load Error: {e}')