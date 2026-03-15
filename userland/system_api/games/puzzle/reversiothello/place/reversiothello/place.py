# Generated method: ReversiOthello.place
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class ReversiOthello:
    def place(self, r, c):
        self.board[r][c] = self.turn
        self.moves = int(self.moves) + 1
        self.turn = 3 - int(self.turn)