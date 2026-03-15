"""
Auto-split from userland\system_api\games\puzzle.py — Nonogram._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class Nonogram:
    def _init_state(self):
        self.solution = [[1, 0, 1, 0, 1]] * 5
        self.grid = [[None] * 5 for _ in range(5)]
