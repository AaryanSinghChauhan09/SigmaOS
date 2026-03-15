"""
Auto-split from userland\system_api\games\classic.py — ChromaticCrush._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class ChromaticCrush:
    def _init_state(self):
        self.grid = [[random.randint(1, 5) for _ in range(8)] for _ in range(8)]
