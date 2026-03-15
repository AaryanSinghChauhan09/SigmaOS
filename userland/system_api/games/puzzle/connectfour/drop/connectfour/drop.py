# Generated method: ConnectFour.drop
import random
import time
from typing import List, Tuple, Dict, Any, Optional, Set
from .base import SigmaGame

class ConnectFour:
    def drop(self, c: int):
        if not 0 <= c < 7:
            return 'Invalid.'
        for r in range(5, -1, -1):
            if self.board[r][c] == 0:
                self.board[r][c] = self.turn
                self.moves = int(self.moves) + 1
                self.turn = 3 - int(self.turn)
                return 'Dropped.'
        return 'Full.'