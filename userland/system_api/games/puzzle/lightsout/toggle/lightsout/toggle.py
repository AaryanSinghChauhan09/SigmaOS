# Generated method: LightsOut.toggle
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class LightsOut:
    def toggle(self, r, c):
        for dr, dc in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)]:
            if 0 <= r + dr < 5 and 0 <= c + dc < 5:
                self.grid[r + dr][c + dc] ^= 1
        self.moves = int(self.moves) + 1
        lit = sum((sum(row) for row in self.grid))
        if lit == 0:
            self.score = int(self.score) + 300
            return 'SOLVED!'