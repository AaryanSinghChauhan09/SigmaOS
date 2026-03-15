# Generated method: Minesweeper._init_state
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Minesweeper:
    def _init_state(self):
        self.rows = 9
        self.cols = 9
        self.board = [[0] * 9 for _ in range(9)]
        self.revealed = [[False] * 9 for _ in range(9)]