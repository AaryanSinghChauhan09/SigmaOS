# Generated method: Nonogram.fill
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Nonogram:
    def fill(self, r, c, val):
        self.grid[r][c] = val
        self.moves = int(self.moves) + 1