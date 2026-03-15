# Generated method: VectorMemory.save
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def save(self):
        """Serializes vectors and metadata to disk."""
        try:
            with open(self.storage_path, 'wb') as f:
                f.write(len(self.vectors).to_bytes(4, 'little'))
                for vec in self.vectors:
                    vec.tofile(f)
            with open(self.storage_path + '.meta', 'w') as f:
                json.dump(self.metadata, f)
        except Exception as e:
            print(f'VectorMemory Save Error: {e}')