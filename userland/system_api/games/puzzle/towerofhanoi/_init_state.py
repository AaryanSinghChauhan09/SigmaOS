"""
Auto-split from userland\system_api\games\puzzle.py — TowerOfHanoi._init_state
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class TowerOfHanoi:
    def _init_state(self):
        self.n = 5
        self.pegs = {'A': list(range(5, 0, -1)), 'B': [], 'C': []}
