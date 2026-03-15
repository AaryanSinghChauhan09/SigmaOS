# Generated method: Minesweeper.reveal
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class Minesweeper:
    def reveal(self, r, c):
        if not (0 <= r < 9 and 0 <= c < 9):
            return 'Out.'
        self.revealed[r][c] = True
        self.moves = int(self.moves) + 1
        return f'At ({r},{c}): {self.board[r][c]}'