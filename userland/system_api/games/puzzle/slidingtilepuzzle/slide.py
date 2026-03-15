"""
Auto-split from userland\system_api\games\puzzle.py — SlidingTilePuzzle.slide
"""

import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame



class SlidingTilePuzzle:
    def slide(self, direction: str) -> str:
        n = int(self.size)
        b = self.board[:]
        blank = int(self.idx)
        m = {'up': blank + n, 'down': blank - n, 'left': blank + 1, 'right': blank - 1}
        t = m.get(direction, -1)
        if 0 <= t < n * n:
            b[blank], b[t] = (b[t], b[blank])
            self.board = b
            self.idx = t
            self.moves = int(self.moves) + 1
            if b == list(range(n * n)):
                self.score = int(self.score) + 500
                return 'SOLVED!'
        return f'Moves: {self.moves}'
