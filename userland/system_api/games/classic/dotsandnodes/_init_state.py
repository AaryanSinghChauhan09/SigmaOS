"""
Auto-split from userland\system_api\games\classic.py — DotsAndNodes._init_state
"""

import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame



class DotsAndNodes:
    def _init_state(self, size=4):
        self.size = size
        self.h_lines = [[False] * size for _ in range(size + 1)]
        self.v_lines = [[False] * (size + 1) for _ in range(size)]
        self.scores = {'A': 0, 'B': 0}
        self.turn = 'A'
