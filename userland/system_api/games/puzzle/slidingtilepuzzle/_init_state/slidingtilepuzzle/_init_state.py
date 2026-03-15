# Generated method: SlidingTilePuzzle._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class SlidingTilePuzzle:
    def _init_state(self):
        self.size = 4
        n = 16
        tiles = list(range(n))
        random.shuffle(tiles)
        self.board = tiles
        self.idx = tiles.index(0)