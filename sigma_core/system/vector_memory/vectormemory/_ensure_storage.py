# Generated method: VectorMemory._ensure_storage
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def _ensure_storage(self):
        os.makedirs(os.path.dirname(self.storage_path), exist_ok=True)