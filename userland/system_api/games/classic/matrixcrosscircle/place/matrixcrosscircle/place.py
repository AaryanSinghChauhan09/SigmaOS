# Generated method: MatrixCrossCircle.place
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class MatrixCrossCircle:
    def place(self, r, c):
        if self.board[r][c] == '.':
            self.board[r][c] = self.turn
            self.moves = int(self.moves) + 1
            self.turn = 'O' if self.turn == 'X' else 'X'