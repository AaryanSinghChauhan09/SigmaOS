"""
Auto-split from userland\system_api\games\puzzle.py — Battleship.fire
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class Battleship:
    def fire(self, r: int, c: int) -> str:
        self.moves = int(self.moves) + 1
        if self.grid[r][c]:
            self.hits.append((r, c))
            self.score = int(self.score) + 20
            return '💥 HIT!'
        self.misses.append((r, c))
        return '💦 MISS'
