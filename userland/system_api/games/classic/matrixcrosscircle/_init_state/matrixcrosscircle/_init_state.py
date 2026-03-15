# Generated method: MatrixCrossCircle._init_state
import random
from typing import List, Tuple, Dict, Any, Optional
from .base import SigmaGame

class MatrixCrossCircle:
    def _init_state(self, size=3):
        self.size = size
        self.board = [['.' for _ in range(size)] for _ in range(size)]
        self.turn = 'X'